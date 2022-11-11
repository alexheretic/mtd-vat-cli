# HMRC application guide
To use _mtd-vat-cli_ you need an HMRC `CLIENT_ID` & `CLIENT_SECRET` associated with an approved app subscribed to the **VAT (MTD)** API. To get that register at the [HMRC developer hub](https://developer.service.hmrc.gov.uk).

I use _mtd-vat-cli_ successfully in an approved app but my client credentials cannot be shared so each user must apply for their own.

## Redirect URIs
The app uses the following redirect url as part of the auth flow.
* `http://localhost:54786/`

## Application questions & answers
Many of the questions in an app application will relate to the _mtd-vat-cli_ software itself. Assuming the intended use of the software is direct, the following answers may be used:

* Will you sell, resell or distribute your software?: **No**
* Do you use HMRC logos in your software, marketing or website?: **No**
* Do your development practices follow our guidance?: **Yes**
* Does your error handling meet our specification?: **Yes**
* Does your software meet accessibility standards?: **Yes**
* Do you have a process for notifying HMRC in the case of a security breach?: **Yes**
* Do you comply with the UK General Data Protection Regulation (UK GDPR)?: **Yes**
* Do you encrypt all customer data that you handle?: **Yes**
* Do you provide software as a service (SaaS)?: **No**
* Does your software submit fraud prevention data?: **Yes**
* Have you checked that your software submits fraud prevention data correctly?: **Yes**
* Confirm the name of your software: **mtd-vat-cli**
* What is your privacy policy URL?: https://github.com/alexheretic/mtd-vat-cli/blob/main/PRIVACY.md
* What is your terms and conditions URL?: https://github.com/alexheretic/mtd-vat-cli/blob/main/LICENSE

## Sandbox
Part of the approval process is testing the app in HMRC's sandbox environment. _mtd-vat-cli_ can target the sandbox APIs by compiling the program with the feature `sandbox`.

E.g. checkout the project and run:
```sh
export CLIENT_ID=$SANDBOX_CLIENT_ID
export CLIENT_SECRET=$SANDBOX_CLIENT_SECRET

cargo run --features sandbox -- --vrn $SANDBOX_VRN
```

## Fraud prevention headers
_mtd-vat-cli_ supports sending fraud prevention headers. You may need to use the software in sandbox to prove this as part of the application.

Testing this involves calling `/test/fraud-prevention-headers/vat-mtd/validation-feedback` API after using the app in sandbox mode.

Issues:
* Header `gov-client-multi-factor` is not included since _mtd-vat-cli_ does not use multi-factor auth. You may be required to state this at some point before being approved.
* Header `gov-client-user-agent` device information is only currently fully populated on Linux OS.
