import { Buffer } from "buffer";
import { Address } from '@stellar/stellar-sdk';
import {
  AssembledTransaction,
  Client as ContractClient,
  ClientOptions as ContractClientOptions,
  MethodOptions,
  Result,
  Spec as ContractSpec,
} from '@stellar/stellar-sdk/contract';
import type {
  u32,
  i32,
  u64,
  i64,
  u128,
  i128,
  u256,
  i256,
  Option,
  Typepoint,
  Duration,
} from '@stellar/stellar-sdk/contract';
export * from '@stellar/stellar-sdk'
export * as contract from '@stellar/stellar-sdk/contract'
export * as rpc from '@stellar/stellar-sdk/rpc'

if (typeof window !== 'undefined') {
  //@ts-ignore Buffer exists
  window.Buffer = window.Buffer || Buffer;
}


export const networks = {
  testnet: {
    networkPassphrase: "Test SDF Network ; September 2015",
    contractId: "CDMOTWEVGBLFKHNE766GWRKXCREQLIOTVPG7RV5DQMM2AJHAVYEIREWO",
  }
} as const

/**
 * Claves de almacenamiento del contrato
 * 
 * Separamos en dos tipos:
 * - Instance Storage: Metadata global (Admin, Name, Symbol, etc.)
 * - Persistent Storage: Datos de usuarios (Balance, Allowance)
 */
export type DataKey = {tag: "Balance", values: readonly [string]} | {tag: "Allowance", values: readonly [string, string]} | {tag: "TotalSupply", values: void} | {tag: "Admin", values: void} | {tag: "TokenName", values: void} | {tag: "TokenSymbol", values: void} | {tag: "Decimals", values: void} | {tag: "Initialized", values: void};

/**
 * Errores personalizados del token
 * 
 * Cada error tiene un código único que se verá en los logs
 * de Stellar cuando una transacción falle
 */
export const TokenError = {
  /**
   * El contrato ya fue inicializado
   */
  1: {message:"AlreadyInitialized"},
  /**
   * Amount debe ser > 0 (o >= 0 para approve)
   */
  2: {message:"InvalidAmount"},
  /**
   * Balance insuficiente para la operación
   */
  3: {message:"InsufficientBalance"},
  /**
   * Allowance insuficiente para transfer_from
   */
  4: {message:"InsufficientAllowance"},
  /**
   * El contrato no ha sido inicializado
   */
  5: {message:"NotInitialized"},
  /**
   * Metadata inválido (name, symbol, decimals)
   */
  6: {message:"InvalidMetadata"},
  /**
   * Overflow en operación aritmética
   */
  7: {message:"Overflow"},
  /**
   * Transferencia a la misma cuenta (from == to)
   */
  8: {message:"SameAccount"}
}






export interface Client {
  /**
   * Construct and simulate a initialize transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  initialize: ({admin, name, symbol, decimals}: {admin: string, name: string, symbol: string, decimals: u32}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Result<void>>>

  /**
   * Construct and simulate a mint transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  mint: ({to, amount}: {to: string, amount: i128}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Result<void>>>

  /**
   * Construct and simulate a burn transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  burn: ({from, amount}: {from: string, amount: i128}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Result<void>>>

  /**
   * Construct and simulate a balance transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  balance: ({id}: {id: string}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<i128>>

  /**
   * Construct and simulate a transfer transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  transfer: ({from, to, amount}: {from: string, to: string, amount: i128}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Result<void>>>

  /**
   * Construct and simulate a approve transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  approve: ({from, spender, amount, live_until_ledger}: {from: string, spender: string, amount: i128, live_until_ledger: u32}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Result<void>>>

  /**
   * Construct and simulate a allowance transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  allowance: ({from, spender}: {from: string, spender: string}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<i128>>

  /**
   * Construct and simulate a transfer_from transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  transfer_from: ({spender, from, to, amount}: {spender: string, from: string, to: string, amount: i128}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Result<void>>>

  /**
   * Construct and simulate a name transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  name: (options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<string>>

  /**
   * Construct and simulate a symbol transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  symbol: (options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<string>>

  /**
   * Construct and simulate a decimals transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  decimals: (options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<u32>>

  /**
   * Construct and simulate a total_supply transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  total_supply: (options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<i128>>

  /**
   * Construct and simulate a admin transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  admin: (options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<string>>

}
export class Client extends ContractClient {
  static async deploy<T = Client>(
    /** Options for initializing a Client as well as for calling a method, with extras specific to deploying. */
    options: MethodOptions &
      Omit<ContractClientOptions, "contractId"> & {
        /** The hash of the Wasm blob, which must already be installed on-chain. */
        wasmHash: Buffer | string;
        /** Salt used to generate the contract's ID. Passed through to {@link Operation.createCustomContract}. Default: random. */
        salt?: Buffer | Uint8Array;
        /** The format used to decode `wasmHash`, if it's provided as a string. */
        format?: "hex" | "base64";
      }
  ): Promise<AssembledTransaction<T>> {
    return ContractClient.deploy(null, options)
  }
  constructor(public readonly options: ContractClientOptions) {
    super(
      new ContractSpec([ "AAAAAgAAALtDbGF2ZXMgZGUgYWxtYWNlbmFtaWVudG8gZGVsIGNvbnRyYXRvCgpTZXBhcmFtb3MgZW4gZG9zIHRpcG9zOgotIEluc3RhbmNlIFN0b3JhZ2U6IE1ldGFkYXRhIGdsb2JhbCAoQWRtaW4sIE5hbWUsIFN5bWJvbCwgZXRjLikKLSBQZXJzaXN0ZW50IFN0b3JhZ2U6IERhdG9zIGRlIHVzdWFyaW9zIChCYWxhbmNlLCBBbGxvd2FuY2UpAAAAAAAAAAAHRGF0YUtleQAAAAAIAAAAAQAAAC5CYWxhbmNlIGRlIHVuIHVzdWFyaW8gZXNwZWPDrWZpY28gLSBQZXJzaXN0ZW50AAAAAAAHQmFsYW5jZQAAAAABAAAAEwAAAAEAAAA5QWxsb3dhbmNlIGVudHJlIGRvcyBjdWVudGFzIChvd25lciwgc3BlbmRlcikgLSBQZXJzaXN0ZW50AAAAAAAACUFsbG93YW5jZQAAAAAAAAIAAAATAAAAEwAAAAAAAAAhU3VwcGx5IHRvdGFsIGRlIHRva2VucyAtIEluc3RhbmNlAAAAAAAAC1RvdGFsU3VwcGx5AAAAAAAAAAAnRGlyZWNjacOzbiBkZWwgYWRtaW5pc3RyYWRvciAtIEluc3RhbmNlAAAAAAVBZG1pbgAAAAAAAAAAAAAbTm9tYnJlIGRlbCB0b2tlbiAtIEluc3RhbmNlAAAAAAlUb2tlbk5hbWUAAAAAAAAAAAAAHVPDrW1ib2xvIGRlbCB0b2tlbiAtIEluc3RhbmNlAAAAAAAAC1Rva2VuU3ltYm9sAAAAAAAAAAAfTsO6bWVybyBkZSBkZWNpbWFsZXMgLSBJbnN0YW5jZQAAAAAIRGVjaW1hbHMAAAAAAAAAIkZsYWcgZGUgaW5pY2lhbGl6YWNpw7NuIC0gSW5zdGFuY2UAAAAAAAtJbml0aWFsaXplZAA=",
        "AAAABAAAAIZFcnJvcmVzIHBlcnNvbmFsaXphZG9zIGRlbCB0b2tlbgoKQ2FkYSBlcnJvciB0aWVuZSB1biBjw7NkaWdvIMO6bmljbyBxdWUgc2UgdmVyw6EgZW4gbG9zIGxvZ3MKZGUgU3RlbGxhciBjdWFuZG8gdW5hIHRyYW5zYWNjacOzbiBmYWxsZQAAAAAAAAAAAApUb2tlbkVycm9yAAAAAAAIAAAAH0VsIGNvbnRyYXRvIHlhIGZ1ZSBpbmljaWFsaXphZG8AAAAAEkFscmVhZHlJbml0aWFsaXplZAAAAAAAAQAAAClBbW91bnQgZGViZSBzZXIgPiAwIChvID49IDAgcGFyYSBhcHByb3ZlKQAAAAAAAA1JbnZhbGlkQW1vdW50AAAAAAAAAgAAACdCYWxhbmNlIGluc3VmaWNpZW50ZSBwYXJhIGxhIG9wZXJhY2nDs24AAAAAE0luc3VmZmljaWVudEJhbGFuY2UAAAAAAwAAAClBbGxvd2FuY2UgaW5zdWZpY2llbnRlIHBhcmEgdHJhbnNmZXJfZnJvbQAAAAAAABVJbnN1ZmZpY2llbnRBbGxvd2FuY2UAAAAAAAAEAAAAI0VsIGNvbnRyYXRvIG5vIGhhIHNpZG8gaW5pY2lhbGl6YWRvAAAAAA5Ob3RJbml0aWFsaXplZAAAAAAABQAAACtNZXRhZGF0YSBpbnbDoWxpZG8gKG5hbWUsIHN5bWJvbCwgZGVjaW1hbHMpAAAAAA9JbnZhbGlkTWV0YWRhdGEAAAAABgAAACJPdmVyZmxvdyBlbiBvcGVyYWNpw7NuIGFyaXRtw6l0aWNhAAAAAAAIT3ZlcmZsb3cAAAAHAAAALFRyYW5zZmVyZW5jaWEgYSBsYSBtaXNtYSBjdWVudGEgKGZyb20gPT0gdG8pAAAAC1NhbWVBY2NvdW50AAAAAAg=",
        "AAAABQAAAAAAAAAAAAAACUluaXRFdmVudAAAAAAAAAEAAAAKaW5pdF9ldmVudAAAAAAABAAAAAAAAAAFYWRtaW4AAAAAAAATAAAAAAAAAAAAAAAEbmFtZQAAABAAAAAAAAAAAAAAAAZzeW1ib2wAAAAAABAAAAAAAAAAAAAAAAhkZWNpbWFscwAAAAQAAAAAAAAAAg==",
        "AAAABQAAAAAAAAAAAAAACU1pbnRFdmVudAAAAAAAAAEAAAAKbWludF9ldmVudAAAAAAAAwAAAAAAAAAFYWRtaW4AAAAAAAATAAAAAAAAAAAAAAACdG8AAAAAABMAAAAAAAAAAAAAAAZhbW91bnQAAAAAAAsAAAAAAAAAAg==",
        "AAAABQAAAAAAAAAAAAAACUJ1cm5FdmVudAAAAAAAAAEAAAAKYnVybl9ldmVudAAAAAAAAgAAAAAAAAAEZnJvbQAAABMAAAAAAAAAAAAAAAZhbW91bnQAAAAAAAsAAAAAAAAAAg==",
        "AAAABQAAAAAAAAAAAAAADVRyYW5zZmVyRXZlbnQAAAAAAAABAAAADnRyYW5zZmVyX2V2ZW50AAAAAAADAAAAAAAAAARmcm9tAAAAEwAAAAAAAAAAAAAAAnRvAAAAAAATAAAAAAAAAAAAAAAGYW1vdW50AAAAAAALAAAAAAAAAAI=",
        "AAAABQAAAAAAAAAAAAAADEFwcHJvdmVFdmVudAAAAAEAAAANYXBwcm92ZV9ldmVudAAAAAAAAAQAAAAAAAAABGZyb20AAAATAAAAAAAAAAAAAAAHc3BlbmRlcgAAAAATAAAAAAAAAAAAAAAGYW1vdW50AAAAAAALAAAAAAAAAAAAAAARbGl2ZV91bnRpbF9sZWRnZXIAAAAAAAAEAAAAAAAAAAI=",
        "AAAAAAAAAAAAAAAKaW5pdGlhbGl6ZQAAAAAABAAAAAAAAAAFYWRtaW4AAAAAAAATAAAAAAAAAARuYW1lAAAAEAAAAAAAAAAGc3ltYm9sAAAAAAAQAAAAAAAAAAhkZWNpbWFscwAAAAQAAAABAAAD6QAAA+0AAAAAAAAH0AAAAApUb2tlbkVycm9yAAA=",
        "AAAAAAAAAAAAAAAEbWludAAAAAIAAAAAAAAAAnRvAAAAAAATAAAAAAAAAAZhbW91bnQAAAAAAAsAAAABAAAD6QAAA+0AAAAAAAAH0AAAAApUb2tlbkVycm9yAAA=",
        "AAAAAAAAAAAAAAAEYnVybgAAAAIAAAAAAAAABGZyb20AAAATAAAAAAAAAAZhbW91bnQAAAAAAAsAAAABAAAD6QAAA+0AAAAAAAAH0AAAAApUb2tlbkVycm9yAAA=",
        "AAAAAAAAAAAAAAAHYmFsYW5jZQAAAAABAAAAAAAAAAJpZAAAAAAAEwAAAAEAAAAL",
        "AAAAAAAAAAAAAAAIdHJhbnNmZXIAAAADAAAAAAAAAARmcm9tAAAAEwAAAAAAAAACdG8AAAAAABMAAAAAAAAABmFtb3VudAAAAAAACwAAAAEAAAPpAAAD7QAAAAAAAAfQAAAAClRva2VuRXJyb3IAAA==",
        "AAAAAAAAAAAAAAAHYXBwcm92ZQAAAAAEAAAAAAAAAARmcm9tAAAAEwAAAAAAAAAHc3BlbmRlcgAAAAATAAAAAAAAAAZhbW91bnQAAAAAAAsAAAAAAAAAEWxpdmVfdW50aWxfbGVkZ2VyAAAAAAAABAAAAAEAAAPpAAAD7QAAAAAAAAfQAAAAClRva2VuRXJyb3IAAA==",
        "AAAAAAAAAAAAAAAJYWxsb3dhbmNlAAAAAAAAAgAAAAAAAAAEZnJvbQAAABMAAAAAAAAAB3NwZW5kZXIAAAAAEwAAAAEAAAAL",
        "AAAAAAAAAAAAAAANdHJhbnNmZXJfZnJvbQAAAAAAAAQAAAAAAAAAB3NwZW5kZXIAAAAAEwAAAAAAAAAEZnJvbQAAABMAAAAAAAAAAnRvAAAAAAATAAAAAAAAAAZhbW91bnQAAAAAAAsAAAABAAAD6QAAA+0AAAAAAAAH0AAAAApUb2tlbkVycm9yAAA=",
        "AAAAAAAAAAAAAAAEbmFtZQAAAAAAAAABAAAAEA==",
        "AAAAAAAAAAAAAAAGc3ltYm9sAAAAAAAAAAAAAQAAABA=",
        "AAAAAAAAAAAAAAAIZGVjaW1hbHMAAAAAAAAAAQAAAAQ=",
        "AAAAAAAAAAAAAAAMdG90YWxfc3VwcGx5AAAAAAAAAAEAAAAL",
        "AAAAAAAAAAAAAAAFYWRtaW4AAAAAAAAAAAAAAQAAABM=" ]),
      options
    )
  }
  public readonly fromJSON = {
    initialize: this.txFromJSON<Result<void>>,
        mint: this.txFromJSON<Result<void>>,
        burn: this.txFromJSON<Result<void>>,
        balance: this.txFromJSON<i128>,
        transfer: this.txFromJSON<Result<void>>,
        approve: this.txFromJSON<Result<void>>,
        allowance: this.txFromJSON<i128>,
        transfer_from: this.txFromJSON<Result<void>>,
        name: this.txFromJSON<string>,
        symbol: this.txFromJSON<string>,
        decimals: this.txFromJSON<u32>,
        total_supply: this.txFromJSON<i128>,
        admin: this.txFromJSON<string>
  }
}