import { Tags } from 'aws-cdk-lib';
import { SSTConfig } from 'sst';
import { Website } from './stacks/Website';

export const APP_NAME = 'mate-bar';

export default {
  config(_input) {
    return {
      name: 'etc',
      region: 'eu-west-1',
    };
  },
  stacks(app) {
    Tags.of(app).add('project', APP_NAME);
    Tags.of(app).add('env', app.stage);

    const isProd = app.stage === 'prod';
    if (!isProd) {
      app.setDefaultRemovalPolicy('destroy');
    }

    app.stack(Website);
  },
} satisfies SSTConfig;
