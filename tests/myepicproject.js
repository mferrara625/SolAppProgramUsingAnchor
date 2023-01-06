const anchor = require("@project-serum/anchor");

const { SystemProgram } = anchor.web3;

const main = async() => {
  console.log("Starting test...")

  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Myepicproject;

  const baseAccount = anchor.web3.Keypair.generate();

  const tx = await program.rpc.initialize({
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
      systemProgram: SystemProgram.programId
    },
    signers: [baseAccount],
  });

  console.log("Your transaction signature", tx)

  let account = await program.account.baseAccount.fetch(baseAccount.publicKey);

  console.log("Selfie Count", account.totalSelfies.toString())

  await program.rpc.addSelfie("https://mferr.s3.amazonaws.com/testSelfie4.jpg",{
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
    },
  });

  account = await program.account.baseAccount.fetch(baseAccount.publicKey);

  console.log("Selfie Count", account.totalSelfies.toString())

  console.log('Selfie List', account.selfieList)


}

const runMain = async() => {
  try{
    await main();
    process.exit(0);
  } catch (error) {
    console.error(error);
    process.exit(1);
  }
}

runMain();
