export interface RequestPayload {
    method: string;
    params: {
        last_n_blocks: number;
        account: string;
    };
    id: number;
    jsonrpc: string;
}

export interface FeeEstimates {
    extreme: number;
    high: number;
    low: number;
    medium: number;
    percentiles: {
        [key: string]: number;
    };
}

export interface ResponseData {
    jsonrpc: string;
    result: {
        context: {
            slot: number;
        };
        per_compute_unit: FeeEstimates;
        per_transaction: FeeEstimates;
    };
    id: number;
}

export interface EstimatePriorityFeesParams {
    last_n_blocks?: number;
    account?: string;
    endpoint: string;
}