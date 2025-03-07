import {join} from 'path';
import {RustFunction} from 'cargo-lambda-cdk';
import {EndpointType, LambdaRestApi} from 'aws-cdk-lib/aws-apigateway'
import {AttributeType, BillingMode, Table, StreamViewType} from 'aws-cdk-lib/aws-dynamodb';
import {RemovalPolicy, Stack, StackProps} from "aws-cdk-lib";
import {Construct} from "constructs";

export interface ECLFStackProps extends StackProps {
  signatureServiceLambdaFunctionName: string;
  emailConfirmationDynamoTableName: string;
}

export class CdkStack extends Stack {
  constructor(scope: Construct, id: string, props: ECLFStackProps) {
    super(scope, id, props);

    const dynamoTable = new Table(this, props.emailConfirmationDynamoTableName, {
      partitionKey: { name: 'pk', type: AttributeType.STRING },
      stream: StreamViewType.NEW_AND_OLD_IMAGES,
      billingMode: BillingMode.PAY_PER_REQUEST,
      removalPolicy: RemovalPolicy.RETAIN,
    });

    const lambdaHandler = new RustFunction(this, 'EmailConfirmationLambdaFunction', {
      manifestPath: join(__dirname, '..', '..'),
      environment: {
        "EMAIL_CONFIRMATION_REQUEST_SERVICE_DYNAMO_TABLE_NAME": dynamoTable.tableName,
        "SIGNATURE_SERVICE_LAMBDA_FUNCTION_NAME": props.signatureServiceLambdaFunctionName
      }
    });

    dynamoTable.grantFullAccess(lambdaHandler);

    new LambdaRestApi(this, 'EmailConfirmationLambdaAPIGateway', {
      handler: lambdaHandler,
      endpointTypes: [EndpointType.REGIONAL]
    });
  }
}


