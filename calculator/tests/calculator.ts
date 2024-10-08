import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Calculator } from "../target/types/calculator";
import { expect } from "chai";

describe("calculator", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  // Referencing the program
  const program = anchor.workspace.Calculator as Program<Calculator>;
  const programProvider = program.provider as anchor.AnchorProvider;

  // Generating keypair for our Calculator account
  const calculatorKeyPair = anchor.web3.Keypair.generate();

  const initMsg = "Random text";

  it("Calculator creation", async () => {
    await program.methods.create(initMsg).accounts(
      {
        calculator: calculatorKeyPair.publicKey,
        user: programProvider.wallet.publicKey,
      }
    ).signers([calculatorKeyPair]).rpc()

    const account = await program.account.calculator.fetch(calculatorKeyPair.publicKey);
    expect(account.greeting).to.eql(initMsg);
  });

  it("Calculator addition", async () => {
    await program.methods.add(new anchor.BN(3), new anchor.BN(4)).accounts({
      calculator: calculatorKeyPair.publicKey
    }).rpc()

    const account = await program.account.calculator.fetch(calculatorKeyPair.publicKey);
    expect(account.result).to.eql(new anchor.BN(7));
  })
});
