import * as flatbuffers from 'flatbuffers';
import { ChannelInstructionIxType } from './channel-instruction-ix-type.js';
export declare class ChannelInstruction {
    bb: flatbuffers.ByteBuffer | null;
    bb_pos: number;
    __init(i: number, bb: flatbuffers.ByteBuffer): ChannelInstruction;
    static getRootAsChannelInstruction(bb: flatbuffers.ByteBuffer, obj?: ChannelInstruction): ChannelInstruction;
    static getSizePrefixedRootAsChannelInstruction(bb: flatbuffers.ByteBuffer, obj?: ChannelInstruction): ChannelInstruction;
    ixType(): ChannelInstructionIxType;
    mutate_ix_type(value: ChannelInstructionIxType): boolean;
    executeV1(index: number): number | null;
    executeV1Length(): number;
    executeV1Array(): Uint8Array | null;
    statusV1(index: number): number | null;
    statusV1Length(): number;
    statusV1Array(): Uint8Array | null;
    deployV1(index: number): number | null;
    deployV1Length(): number;
    deployV1Array(): Uint8Array | null;
    claimV1(index: number): number | null;
    claimV1Length(): number;
    claimV1Array(): Uint8Array | null;
    static startChannelInstruction(builder: flatbuffers.Builder): void;
    static addIxType(builder: flatbuffers.Builder, ixType: ChannelInstructionIxType): void;
    static addExecuteV1(builder: flatbuffers.Builder, executeV1Offset: flatbuffers.Offset): void;
    static createExecuteV1Vector(builder: flatbuffers.Builder, data: number[] | Uint8Array): flatbuffers.Offset;
    static startExecuteV1Vector(builder: flatbuffers.Builder, numElems: number): void;
    static addStatusV1(builder: flatbuffers.Builder, statusV1Offset: flatbuffers.Offset): void;
    static createStatusV1Vector(builder: flatbuffers.Builder, data: number[] | Uint8Array): flatbuffers.Offset;
    static startStatusV1Vector(builder: flatbuffers.Builder, numElems: number): void;
    static addDeployV1(builder: flatbuffers.Builder, deployV1Offset: flatbuffers.Offset): void;
    static createDeployV1Vector(builder: flatbuffers.Builder, data: number[] | Uint8Array): flatbuffers.Offset;
    static startDeployV1Vector(builder: flatbuffers.Builder, numElems: number): void;
    static addClaimV1(builder: flatbuffers.Builder, claimV1Offset: flatbuffers.Offset): void;
    static createClaimV1Vector(builder: flatbuffers.Builder, data: number[] | Uint8Array): flatbuffers.Offset;
    static startClaimV1Vector(builder: flatbuffers.Builder, numElems: number): void;
    static endChannelInstruction(builder: flatbuffers.Builder): flatbuffers.Offset;
    static finishChannelInstructionBuffer(builder: flatbuffers.Builder, offset: flatbuffers.Offset): void;
    static finishSizePrefixedChannelInstructionBuffer(builder: flatbuffers.Builder, offset: flatbuffers.Offset): void;
    static createChannelInstruction(builder: flatbuffers.Builder, ixType: ChannelInstructionIxType, executeV1Offset: flatbuffers.Offset, statusV1Offset: flatbuffers.Offset, deployV1Offset: flatbuffers.Offset, claimV1Offset: flatbuffers.Offset): flatbuffers.Offset;
}