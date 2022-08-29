mod auth;
mod io;
mod reqwest_ext;
mod vat;

use clap::Parser;

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

    /// Application client id.
    #[clap(long, env = "CLIENT_ID")]
    pub client_id: String,

    /// Application client secret.
    #[clap(long, env = "CLIENT_SECRET")]
    pub client_secret: String,

    /// Re-run authorize even if token has already been cached.
    #[clap(long)]
    pub reauth: bool,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let Args {
        client_id,
        client_secret,
        vrn,
        reauth,
    } = Args::parse();

    let token = match io::read_token(&vrn).await? {
        // reuse token (todo invalidate when it no longer works)
        Some(token) if !reauth => token,
        _ => {
            let token = auth::user_auth(&client_id, &client_secret).await?;
            io::write_token(&vrn, &token).await?;
            token
        }
    };

    let vat = vat::Client::new(token, vrn);

    let obligations = vat.open_obligations().await?;
    if obligations.is_empty() {
        eprintln!("No open obligations :)");
        return Ok(());
    }

    eprintln!("==> open-obligations");
    for obligation in &obligations {
        eprintln!(
            "  - {} start:{} end:{} due:{}",
            obligation.period_key, obligation.start, obligation.end, obligation.due,
        );
    }
    eprintln!();

    for obligation in obligations {
        let msg = format!("Submit return for {}? [yN] ", obligation.period_key);
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

            let confirm_msg = format!("\n{vreturn:#?}\nSend? [yN] ");
            if prompt_input(&confirm_msg)?.eq_ignore_ascii_case("y") {
                vat.submit_return(&vreturn).await?;
                eprintln!("Ok ✓");
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
