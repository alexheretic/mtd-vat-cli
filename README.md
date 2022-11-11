# mtd-vat-cli
CLI tool to query & submit UK VAT returns via the [VAT (MTD) API](https://developer.service.hmrc.gov.uk/api-documentation/docs/api/service/vat-api/1.0). 

Serves as manual MTD "bridging" software to submit VAT returns.

```
mtd-vat [OPTIONS] --vrn <VRN> --client-id <CLIENT_ID> --client-secret <CLIENT_SECRET>
```

Required args:
* `--vrn` VAT registration number
* `--client-id` or `env CLIENT_ID` HMRC registered app client id.
* `--client-secret` or `env CLIENT_SECRET` HMRC registered app client secret.

To explain how to answer the VAT return questions see [the gov guidance](https://www.gov.uk/guidance/how-to-fill-in-and-submit-your-vat-return-vat-notice-70012).

## Getting a CLIENT_ID
You need to apply for one. See [APPLYING](./APPLYING.md).

## Minimum supported rust compiler
Maintained with [latest stable rust](https://gist.github.com/alexheretic/d1e98d8433b602e57f5d0a9637927e0c).
