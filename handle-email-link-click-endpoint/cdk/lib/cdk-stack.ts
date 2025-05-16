import { join } from 'path';
import { RustFunction } from 'cargo-lambda-cdk';
import { EndpointType, LambdaRestApi } from 'aws-cdk-lib/aws-apigateway'
import { Stack, StackProps } from "aws-cdk-lib";
import { Construct } from "constructs";

export interface HELCLFStackProps extends StackProps {
  emailConfirmationRequestServiceUrl: string;
  emailConfirmationRequestInternalApiKey: string;
  emailLinkClickHandlerServiceUrl: string;
}

export class CdkStack extends Stack {
  constructor(scope: Construct, id: string, props: HELCLFStackProps) {
    super(scope, id, props);

    const lambdaHandler = new RustFunction(this, 'HandleEmailLinkClickLambdaFunction', {
      manifestPath: join(__dirname, '..', '..'),
      environment: {
        "EMAIL_CONFIRMATION_REQUEST_SERVICE_URL": props.emailConfirmationRequestServiceUrl,
        "EMAIL_CONFIRMATION_REQUEST_SERVICE_INTERNAL_API_KEY": props.emailConfirmationRequestInternalApiKey,
        "EMAIL_LINK_CLICK_HANDLER_SERVICE_URL": props.emailLinkClickHandlerServiceUrl
      }
    });

    new LambdaRestApi(this, 'EmailLinkClickHandlerLambdaAPIGateway', {
      handler: lambdaHandler,
      endpointTypes: [EndpointType.REGIONAL]
    });
  }
}


