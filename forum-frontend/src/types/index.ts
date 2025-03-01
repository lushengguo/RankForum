export type Address = string;

export interface User {
    address: Address;
    name: string;
}

export interface Field {
    name: string;
    address: Address;
}

export interface Comment {
    address: Address;
    from: Address;
    to: Address;
    score: string;
    upvote: number;
    downvote: number;
    content: string;
    timestamp: number;
    field_address: Address;
    comments: Comment[];
}

export interface Post {
    address: Address;
    from: Address;
    to: Address;
    title: string;
    content: string;
    score: string;
    upvote: number;
    downvote: number;
    timestamp: number;
    comments: Comment[];
}

export enum OrderingType {
    ByTimestamp = "timestamp",
    ByScore = "score",
    ByUpVote = "upvote",
    ByDownVote = "downvote",
    ByUpvoteSubDownVote = "upvote-downvote"
}

export interface FilterOption {
    level?: number;
    keyword?: string;
    ordering: OrderingType;
    ascending: boolean;
    max_results: number;
}

export interface AuthState {
    isAuthenticated: boolean;
    user: User | null;
    sessionId: string | null;
} 