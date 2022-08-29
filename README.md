# mtd-vat-cli
CLI tool to query & submit UK VAT returns via the [VAT (MTD) API](https://developer.service.hmrc.gov.uk/api-documentation/docs/api/service/vat-api/1.0). 

```sh
mtd-vat --client-id $CLIENT_ID --client-secret=$CLIENT_SECRET --vrn $VRN
```

Serves as manual "bridging" software to submit VAT returns in a MTD compatible way.

To explain how to answer the VAT return questions see [the gov guidance](https://www.gov.uk/guidance/how-to-fill-in-and-submit-your-vat-return-vat-notice-70012).

## Minimum supported rust compiler
Maintained with [latest stable rust](https://gist.github.com/alexheretic/d1e98d8433b602e57f5d0a9637927e0c).
