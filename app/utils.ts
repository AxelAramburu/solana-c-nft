import { Transaction, ComputeBudgetProgram } from "@solana/web3.js";
import { RequestPayload, ResponseData, EstimatePriorityFeesParams } from "./types";


export async function fetchEstimatePriorityFees({
    last_n_blocks,
    account,
    endpoint
}: EstimatePriorityFeesParams): Promise<ResponseData> {
    const params: any = {};
    if (last_n_blocks !== undefined) {
        params.last_n_blocks = last_n_blocks;
    }
    if (account !== undefined) {
        params.account = account;
    }

    const payload: RequestPayload = {
        method: 'qn_estimatePriorityFees',
        params,
        id: 1,
        jsonrpc: '2.0',
    };

    const response = await fetch(endpoint, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify(payload),
    });

    if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`);
    }

    const data: ResponseData = await response.json();
    return data;
}