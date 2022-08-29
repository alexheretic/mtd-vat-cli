mod auth;
mod error;
mod fraud_prevention;
mod vat;

use clap::Parser;
use console::style;

#[cfg(not(feature = "sandbox"))]
const WWW_URL: &str = "https://www.tax.service.gov.uk";
#[cfg(not(feature = "sandbox"))]
const API_URL: &str = "https://api.service.hmrc.gov.uk";

#[cfg(feature = "sandbox")]
const WWW_URL: &str = "https://test-www.tax.service.gov.uk";
#[cfg(feature = "sandbox")]
const API_URL: &str = "https://test-api.service.hmrc.gov.uk";

#[derive(Parser, Clone)]
#[clap(version, about)]
pub struct Args {
    /// VAT registration number.
    #[clap(long)]
    pub vrn: String,

    /// HMRC registered app client id.
    #[clap(long, env = "CLIENT_ID")]
    pub client_id: String,

    /// HMRC registered app client secret.
    #[clap(long, env = "CLIENT_SECRET")]
    pub client_secret: String,

    /// Access token to be re-used from a previous run.
    #[clap(long)]
    pub access_token: Option<String>,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let Args {
        client_id,
        client_secret,
        vrn,
        access_token,
    } = Args::parse();

    let token = match access_token {
        Some(token) => token,
        _ => {
            eprintln!("{}", style("Use browser to permit app access...").yellow());
            let token = auth::user_auth(&client_id, &client_secret)
                .await?
                .access_token;
            eprintln!(
                "{} To re-use token in subsequent runs add arg {}",
                style("✓").green(),
                style(format!("--access-token={token}")).cyan().bold()
            );
            token
        }
    };

    let vat = vat::Client::new(token, vrn);

    let obligations = vat.open_obligations().await?;
    if obligations.is_empty() {
        eprintln!("{}", style("No open obligations :)").green());
        return Ok(());
    }

    eprintln!("{}", style("==> Open obligations").bold());
    for obligation in &obligations {
        eprintln!(
            "{} start:{} end:{} due:{}",
            style(&obligation.period_key).bold(),
            style(&obligation.start).bold(),
            style(&obligation.end).bold(),
            style(&obligation.due).bold(),
        );
    }
    eprintln!();

    for obligation in obligations {
        let msg = format!(
            "Submit return for {}? [yN] ",
            style(&obligation.period_key).bold()
        );
        if prompt_input(&msg)?.eq_ignore_ascii_case("y") {
            let vat_due_sales = prompt_input("- VAT due on sales and other outputs: ")?.parse()?;
            let vat_due_acquisitions = prompt_input(
                "- VAT due in the period on acquisitions of goods\n  \
                made in Northern Ireland from EU Member States: ",
            )?
            .parse()?;
            let total_vat_due: f64 = vat_due_sales + vat_due_acquisitions;
            let vat_reclaimed_curr_period: f64 = prompt_input(
                "- VAT reclaimed in the period on purchases and\n  \
                other inputs (including acquisitions in Northern\n  \
                Ireland from EU member states): ",
            )?
            .parse()?;
            let net_vat_due = (total_vat_due - vat_reclaimed_curr_period).abs();
            let total_value_sales_ex_vat = prompt_input(
                "- Total value of sales and all other outputs\n  \
                excluding any VAT (no pence): ",
            )?
            .parse()?;
            let total_value_purchases_ex_vat = prompt_input(
                "- Total value of purchases and all other inputs\n  \
                excluding any VAT (including exempt purchases, no pence): ",
            )?
            .parse()?;
            let total_value_goods_supplied_ex_vat = prompt_input(
                "- Total value of dispatches of goods and related\n  \
                costs (excluding VAT) from Northern Ireland to\n  \
                EU Member States (no pence): ",
            )?
            .parse()?;
            let total_acquisitions_ex_vat = prompt_input(
                "- Total value of acquisitions of goods and\n  \
                related costs (excluding VAT) made in\n  \
                Northern Ireland from EU Member States (no pence): ",
            )?
            .parse()?;

            let vreturn = vat::Return {
                period_key: obligation.period_key,
                vat_due_sales,
                vat_due_acquisitions,
                total_vat_due,
                vat_reclaimed_curr_period,
                net_vat_due,
                total_value_sales_ex_vat,
                total_value_purchases_ex_vat,
                total_value_goods_supplied_ex_vat,
                total_acquisitions_ex_vat,
                finalised: true,
            };

            let confirm_msg = format!(
                "\n{}\n{} [yN] ",
                style(format!("{vreturn:#?}")).cyan(),
                style("Send?").bold()
            );
            if prompt_input(&confirm_msg)?.eq_ignore_ascii_case("y") {
                vat.submit_return(&vreturn).await?;
                eprintln!("{}", style("Ok ✓").green());
            }
        }
    }

    Ok(())
}

/// Prompt for user input **blocking**.
fn prompt_input(msg: &str) -> anyhow::Result<String> {
    eprint!("{msg}");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    Ok(input.trim().into())
}
