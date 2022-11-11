use crate::{error::ResponseExt, fraud_prevention::FraudPreventionRequestBuilder, API_URL};

pub struct Client {
    http: reqwest::Client,
    access_token: String,
    vrn: String,
}

impl Client {
    pub fn new(access_token: String, vrn: String) -> Self {
        Self {
            http: reqwest::Client::new(),
            access_token,
            vrn,
        }
    }

    pub async fn open_obligations(&self) -> anyhow::Result<Vec<Obligation>> {
        let mut body: Obligations = self
            .http
            .get(&format!(
                "{API_URL}/organisations/vat/{}/obligations",
                self.vrn
            ))
            .query(&[("status", "O")])
            .header("Accept", "application/vnd.hmrc.1.0+json")
            .header("Authorization", format!("Bearer {}", self.access_token))
            .add_fraud_prevention_headers()
            .send()
            .await?
            .error_body_for_status()
            .await?
            .json()
            .await?;

        body.obligations.sort_by(|a, b| a.start.cmp(&b.start));
        Ok(body.obligations)
    }

    pub async fn submit_return(&self, vreturn: &Return) -> anyhow::Result<()> {
        self.http
            .post(&format!("{API_URL}/organisations/vat/{}/returns", self.vrn))
            .header("Accept", "application/vnd.hmrc.1.0+json")
            .header("Authorization", format!("Bearer {}", self.access_token))
            .add_fraud_prevention_headers()
            .json(vreturn)
            .send()
            .await?
            .error_body_for_status()
            .await?;
        Ok(())
    }
}

#[derive(Debug, serde::Deserialize)]
struct Obligations {
    obligations: Vec<Obligation>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Obligation {
    /// The start date of this obligation period (YYYY-MM-DD).
    pub start: String,
    /// The end date of this obligation period (YYYY-MM-DD).
    pub end: String,
    /// The due date for this obligation period, in the format YYYY-MM-DD.
    /// For example: 2017-01-25. The due date for monthly/quarterly obligations is one month
    /// and seven days from the end date. The due date for Payment On Account customers is
    /// the last working day of the month after the end date. For example if the end date
    /// is 2018-02-28, the due date is 2018-03-29 (because the 31 March is a Saturday
    /// and the 30 March is Good Friday).
    pub due: String,
    /// Which obligation statuses to return (O = Open, F = Fulfilled).
    pub status: char,
    /// The ID code for the period that this obligation belongs to.
    /// The format is a string of four alphanumeric characters.
    pub period_key: String,
    // /// The obligation received date, is returned when status is (F = Fulfilled).
    // received: Option<String>,
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Return {
    /// The ID code for the period that this obligation belongs to.
    /// The format is a string of four alphanumeric characters.
    pub period_key: String,
    /// VAT due on sales and other outputs. This corresponds to box 1 on the VAT Return form.
    pub vat_due_sales: f64,
    /// VAT due in the period on acquisitions of goods made in Northern Ireland from EU Member States.
    /// This corresponds to box 2 on the VAT Return form
    pub vat_due_acquisitions: f64,
    /// Total VAT due (the sum of vatDueSales and vatDueAcquisitions).
    /// This corresponds to box 3 on the VAT Return form.
    pub total_vat_due: f64,
    /// VAT reclaimed in the period on purchases and other inputs
    /// (including acquisitions in Northern Ireland from EU member states).
    /// This corresponds to box 4 on the VAT Return form.
    pub vat_reclaimed_curr_period: f64,
    /// The difference between totalVatDue and vatReclaimedCurrPeriod.
    /// This corresponds to box 5 on the VAT Return form.
    pub net_vat_due: f64,
    /// Total value of sales and all other outputs excluding any VAT.
    /// This corresponds to box 6 on the VAT Return form
    #[serde(rename = "totalValueSalesExVAT")]
    pub total_value_sales_ex_vat: i64,
    /// Total value of purchases and all other inputs excluding any VAT (including exempt purchases).
    /// This corresponds to box 7 on the VAT Return form. The value must be in pounds (no pence).
    #[serde(rename = "totalValuePurchasesExVAT")]
    pub total_value_purchases_ex_vat: i64,
    /// Total value of dispatches of goods and related costs (excluding VAT) from Northern Ireland
    /// to EU Member States. This corresponds to box 8 on the VAT Return form.
    /// The value must be in pounds (no pence).
    #[serde(rename = "totalValueGoodsSuppliedExVAT")]
    pub total_value_goods_supplied_ex_vat: i64,
    /// Total value of acquisitions of goods and related costs (excluding VAT) made in Northern
    /// Ireland from EU Member States. This corresponds to box 9 on the VAT Return form.
    /// The value must be in pounds (no pence).
    #[serde(rename = "totalAcquisitionsExVAT")]
    pub total_acquisitions_ex_vat: i64,
    /// Declaration that the user has finalised their VAT return.
    pub finalised: bool,
}
