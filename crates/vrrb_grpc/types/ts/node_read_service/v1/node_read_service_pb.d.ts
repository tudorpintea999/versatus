// @generated by protoc-gen-es v1.2.0
// @generated from file node_read_service/v1/node_read_service.proto (package node_read_service.v1, syntax proto3)
/* eslint-disable */
// @ts-nocheck

import type { BinaryReadOptions, FieldList, JsonReadOptions, JsonValue, PartialMessage, PlainMessage } from "@bufbuild/protobuf";
import { Message, proto3 } from "@bufbuild/protobuf";

/**
 * @generated from message node_read_service.v1.GetNodeTypeRequest
 */
export declare class GetNodeTypeRequest extends Message<GetNodeTypeRequest> {
  constructor(data?: PartialMessage<GetNodeTypeRequest>);

  static readonly runtime: typeof proto3;
  static readonly typeName = "node_read_service.v1.GetNodeTypeRequest";
  static readonly fields: FieldList;

  static fromBinary(bytes: Uint8Array, options?: Partial<BinaryReadOptions>): GetNodeTypeRequest;

  static fromJson(jsonValue: JsonValue, options?: Partial<JsonReadOptions>): GetNodeTypeRequest;

  static fromJsonString(jsonString: string, options?: Partial<JsonReadOptions>): GetNodeTypeRequest;

  static equals(a: GetNodeTypeRequest | PlainMessage<GetNodeTypeRequest> | undefined, b: GetNodeTypeRequest | PlainMessage<GetNodeTypeRequest> | undefined): boolean;
}

/**
 * @generated from message node_read_service.v1.GetNodeTypeResponse
 */
export declare class GetNodeTypeResponse extends Message<GetNodeTypeResponse> {
  /**
   * @generated from field: string id = 1;
   */
  id: string;

  /**
   * @generated from field: string result = 2;
   */
  result: string;

  constructor(data?: PartialMessage<GetNodeTypeResponse>);

  static readonly runtime: typeof proto3;
  static readonly typeName = "node_read_service.v1.GetNodeTypeResponse";
  static readonly fields: FieldList;

  static fromBinary(bytes: Uint8Array, options?: Partial<BinaryReadOptions>): GetNodeTypeResponse;

  static fromJson(jsonValue: JsonValue, options?: Partial<JsonReadOptions>): GetNodeTypeResponse;

  static fromJsonString(jsonString: string, options?: Partial<JsonReadOptions>): GetNodeTypeResponse;

  static equals(a: GetNodeTypeResponse | PlainMessage<GetNodeTypeResponse> | undefined, b: GetNodeTypeResponse | PlainMessage<GetNodeTypeResponse> | undefined): boolean;
}

/**
 * @generated from message node_read_service.v1.GetFullMempoolRequest
 */
export declare class GetFullMempoolRequest extends Message<GetFullMempoolRequest> {
  constructor(data?: PartialMessage<GetFullMempoolRequest>);

  static readonly runtime: typeof proto3;
  static readonly typeName = "node_read_service.v1.GetFullMempoolRequest";
  static readonly fields: FieldList;

  static fromBinary(bytes: Uint8Array, options?: Partial<BinaryReadOptions>): GetFullMempoolRequest;

  static fromJson(jsonValue: JsonValue, options?: Partial<JsonReadOptions>): GetFullMempoolRequest;

  static fromJsonString(jsonString: string, options?: Partial<JsonReadOptions>): GetFullMempoolRequest;

  static equals(a: GetFullMempoolRequest | PlainMessage<GetFullMempoolRequest> | undefined, b: GetFullMempoolRequest | PlainMessage<GetFullMempoolRequest> | undefined): boolean;
}

/**
 * @generated from message node_read_service.v1.GetFullMempoolResponse
 */
export declare class GetFullMempoolResponse extends Message<GetFullMempoolResponse> {
  /**
   * @generated from field: repeated node_read_service.v1.TransactionRecord transaction_records = 1;
   */
  transactionRecords: TransactionRecord[];

  constructor(data?: PartialMessage<GetFullMempoolResponse>);

  static readonly runtime: typeof proto3;
  static readonly typeName = "node_read_service.v1.GetFullMempoolResponse";
  static readonly fields: FieldList;

  static fromBinary(bytes: Uint8Array, options?: Partial<BinaryReadOptions>): GetFullMempoolResponse;

  static fromJson(jsonValue: JsonValue, options?: Partial<JsonReadOptions>): GetFullMempoolResponse;

  static fromJsonString(jsonString: string, options?: Partial<JsonReadOptions>): GetFullMempoolResponse;

  static equals(a: GetFullMempoolResponse | PlainMessage<GetFullMempoolResponse> | undefined, b: GetFullMempoolResponse | PlainMessage<GetFullMempoolResponse> | undefined): boolean;
}

/**
 * @generated from message node_read_service.v1.TransactionRecord
 */
export declare class TransactionRecord extends Message<TransactionRecord> {
  /**
   * @generated from field: string id = 1;
   */
  id: string;

  /**
   * @generated from field: int64 timestamp = 2;
   */
  timestamp: bigint;

  /**
   * @generated from field: string sender_address = 3;
   */
  senderAddress: string;

  /**
   * @generated from field: string sender_public_key = 4;
   */
  senderPublicKey: string;

  /**
   * @generated from field: string receiver_address = 5;
   */
  receiverAddress: string;

  /**
   * @generated from field: node_read_service.v1.Token token = 6;
   */
  token?: Token;

  /**
   * @generated from field: uint64 amount = 7;
   */
  amount: bigint;

  /**
   * @generated from field: string signature = 8;
   */
  signature: string;

  /**
   * @generated from field: map<string, bool> validators = 9;
   */
  validators: { [key: string]: boolean };

  /**
   * @generated from field: uint64 nonce = 10;
   */
  nonce: bigint;

  constructor(data?: PartialMessage<TransactionRecord>);

  static readonly runtime: typeof proto3;
  static readonly typeName = "node_read_service.v1.TransactionRecord";
  static readonly fields: FieldList;

  static fromBinary(bytes: Uint8Array, options?: Partial<BinaryReadOptions>): TransactionRecord;

  static fromJson(jsonValue: JsonValue, options?: Partial<JsonReadOptions>): TransactionRecord;

  static fromJsonString(jsonString: string, options?: Partial<JsonReadOptions>): TransactionRecord;

  static equals(a: TransactionRecord | PlainMessage<TransactionRecord> | undefined, b: TransactionRecord | PlainMessage<TransactionRecord> | undefined): boolean;
}

/**
 * @generated from message node_read_service.v1.Token
 */
export declare class Token extends Message<Token> {
  /**
   * @generated from field: string name = 1;
   */
  name: string;

  /**
   * @generated from field: string symbol = 2;
   */
  symbol: string;

  /**
   * @generated from field: uint32 decimals = 3;
   */
  decimals: number;

  constructor(data?: PartialMessage<Token>);

  static readonly runtime: typeof proto3;
  static readonly typeName = "node_read_service.v1.Token";
  static readonly fields: FieldList;

  static fromBinary(bytes: Uint8Array, options?: Partial<BinaryReadOptions>): Token;

  static fromJson(jsonValue: JsonValue, options?: Partial<JsonReadOptions>): Token;

  static fromJsonString(jsonString: string, options?: Partial<JsonReadOptions>): Token;

  static equals(a: Token | PlainMessage<Token> | undefined, b: Token | PlainMessage<Token> | undefined): boolean;
}
