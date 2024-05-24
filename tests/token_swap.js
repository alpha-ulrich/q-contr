const anchor = require('@project-serum/anchor');
const { Token, TOKEN_PROGRAM_ID } = require("@solana/spl-token");

describe('token_swap', () => {
  const provider = anchor.Provider.local();
  anchor.setProvider(provider);
  const program = anchor.workspace.TokenSwap;

  let sourceTokenAccount = null;
  let destinationTokenAccount = null;
  let userTokenAccount = null;
  let mint = null;

  before(async () => {
    // Create a new mint
    mint = await Token.createMint(
      provider.connection,
      provider.wallet.payer,
      provider.wallet.publicKey,
      null,
      9,
      TOKEN_PROGRAM_ID
    );

    // Create token accounts
    sourceTokenAccount = await mint.createAccount(provider.wallet.publicKey);
    destinationTokenAccount = await mint.createAccount(provider.wallet.publicKey);
    userTokenAccount = await mint.createAccount(provider.wallet.publicKey);

    // Mint tokens to the user's token account
    await mint.mintTo(
      userTokenAccount,
      provider.wallet.publicKey,
      [],
      1000000
    );
  });

  it('Swaps tokens', async () => {
    await program.rpc.swap(new anchor.BN(100), {
      accounts: {
        sourceAccount: sourceTokenAccount,
        destinationAccount: destinationTokenAccount,
        user: provider.wallet.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
      },
      signers: [],
    });

    const sourceAccountInfo = await mint.getAccountInfo(sourceTokenAccount);
    const destinationAccountInfo = await mint.getAccountInfo(destinationTokenAccount);

    assert.ok(sourceAccountInfo.amount.toNumber() === 100);
    assert.ok(destinationAccountInfo.amount.toNumber() === 0);
  });
});
