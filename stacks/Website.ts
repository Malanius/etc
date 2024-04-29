import {
  StackContext,
  StaticSite,
  StaticSiteDomainProps,
} from 'sst/constructs';

export function Website({ stack }: StackContext) {
  const customDomain: StaticSiteDomainProps | undefined =
    stack.stage === 'prod'
      ? {
          domainName: 'etc.malanius.dev',
          hostedZone: 'malanius.dev',
        }
      : undefined;

  new StaticSite(stack, 'web', {
    path: 'packages/frontend',
    buildOutput: 'dist',
    buildCommand: 'rm -fr dist/ && dx build --release',
    customDomain,
    waitForInvalidation: stack.stage === 'prod',
    assets: {
      fileOptions: [
        {
          files: '**',
          cacheControl: 'max-age=0,no-cache,no-store,must-revalidate',
        },
      ],
    },
  });
}
