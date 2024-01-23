const { LOCALHOST, tmpLedgerDir } = require('@metaplex-foundation/amman');

module.exports = {
  validator: {
    killRunningValidators: true,
    programs: [
    //   { 
    //     label: 'Token Metadata Program',
    //     programId: programIds.metadata,
    //     deployPath: localDeployPath('mpl_token_metadata')
    //   },
    ],
    jsonRpcUrl: LOCALHOST,
    websocketUrl: '',
    commitment: 'confirmed',
    ledgerDir: './test-ledger',
    resetLedger: true,
    verifyFees: false,
    detached: process.env.CI != null,
    accountsCluster: 'https://api.devnet.solana.com',
    accounts: [
        // prorgams
        {
            label: 'Token Metadata Program',
            accountId:'metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s',
            executable: true,
        },
        {
            label: 'BubbleGum',
            accountId:'BGUMAp9Gq7iTEuizy4pqaxsTyUCBK68MDfK752saRPUY',
            executable: true,
        },
        {
            label: 'NOOB',
            accountId:'noopb9bkMVfRPU8AsbpTUg8AQkHtKwMYZiFUjNRtMmV',
            executable: true,
        },
        {
            label: 'SPL Compression',
            accountId:'cmtDvXumGCrqC1Age74AVPhSRVXJMd8PJS91L8KbNCK',
            executable: true,
        },
        // SPL
        {
            label: 'SPL Token',
            accountId: '8DVvzwVsbetpfgHTw1EWXeJHxQcsZscpheEYnc9DK1FM',
            executable: false,
        },
        {
            label: 'TEST1 ATA',
            accountId: '9kTRspJSfD5xnFXhKX1SMar5bva7YLAMvg7u86MA7Bhg',
            executable: false,
        },
        {
            label: 'TEST2 ATA',
            accountId: '2TqqXDbYsCAiEMjJefK1Mf2jjTpD4DUSjhHSJFszqb2w',
            executable: false,
        },
    ]
  },
  relay: {
    enabled: process.env.CI == null,
    killlRunningRelay: true,
  },
  storage: {
    enabled: process.env.CI == null,
    storageId: 'mock-storage',
    clearOnStart: true,
  },
}