import { createSlice, PayloadAction, createAsyncThunk } from '@reduxjs/toolkit';
import { Comment, FilterOption, Post } from '../types';
import { commentAPI, postAPI, voteAPI } from '../services/api';

interface PostState {
    posts: Post[];
    currentPost: Post | null;
    loading: boolean;
    error: string | null;
}

const initialState: PostState = {
    posts: [],
    currentPost: null,
    loading: false,
    error: null
};

// 获取帖子列表
export const fetchPosts = createAsyncThunk(
    'posts/fetchPosts',
    async ({ field_name, field_address, options }: { field_name?: string, field_address?: string, options?: Partial<FilterOption> }, { rejectWithValue }) => {
        try {
            return await postAPI.filterPosts(field_name, field_address, options);
        } catch (error) {
            return rejectWithValue("获取帖子列表失败");
        }
    }
);

// 获取单个帖子详情
export const fetchPostDetails = createAsyncThunk(
    'posts/fetchPostDetails',
    async (address: string, { rejectWithValue }) => {
        try {
            return await postAPI.getPostDetails(address);
        } catch (error) {
            return rejectWithValue("获取帖子详情失败");
        }
    }
);

// 创建新帖子
export const createPost = createAsyncThunk(
    'posts/createPost',
    async ({ field_name, field_address, title, content }: { field_name: string, field_address: string, title: string, content: string }, { rejectWithValue }) => {
        try {
            await postAPI.createPost(field_name, field_address, title, content);
            // 由于后端不直接返回创建的帖子，我们需要重新获取帖子列表
            return await postAPI.filterPosts(field_name, field_address);
        } catch (error) {
            return rejectWithValue("创建帖子失败");
        }
    }
);

// 发表评论
export const createComment = createAsyncThunk(
    'posts/createComment',
    async ({ to, content, field_address }: { to: string, content: string, field_address: string }, { rejectWithValue, dispatch, getState }) => {
        try {
            await commentAPI.createComment(to, content, field_address);
            // 如果当前有帖子被加载，则重新获取帖子详情以更新评论
            const state = getState() as { posts: PostState };
            if (state.posts.currentPost) {
                dispatch(fetchPostDetails(state.posts.currentPost.address));
            }
            return { success: true };
        } catch (error) {
            return rejectWithValue("发表评论失败");
        }
    }
);

// 点赞帖子或评论
export const upvoteItem = createAsyncThunk(
    'posts/upvote',
    async ({ target_address, field_address }: { target_address: string, field_address: string }, { rejectWithValue, dispatch, getState }) => {
        try {
            await voteAPI.upvote(target_address, field_address);
            // 如果当前有帖子被加载，则重新获取帖子详情以更新评分
            const state = getState() as { posts: PostState };
            if (state.posts.currentPost) {
                dispatch(fetchPostDetails(state.posts.currentPost.address));
            }
            return { target_address, success: true };
        } catch (error) {
            return rejectWithValue("点赞失败");
        }
    }
);

// 踩帖子或评论
export const downvoteItem = createAsyncThunk(
    'posts/downvote',
    async ({ target_address, field_address }: { target_address: string, field_address: string }, { rejectWithValue, dispatch, getState }) => {
        try {
            await voteAPI.downvote(target_address, field_address);
            // 如果当前有帖子被加载，则重新获取帖子详情以更新评分
            const state = getState() as { posts: PostState };
            if (state.posts.currentPost) {
                dispatch(fetchPostDetails(state.posts.currentPost.address));
            }
            return { target_address, success: true };
        } catch (error) {
            return rejectWithValue("踩失败");
        }
    }
);

const postSlice = createSlice({
    name: 'posts',
    initialState,
    reducers: {
        clearCurrentPost(state) {
            state.currentPost = null;
        },
        clearPosts(state) {
            state.posts = [];
        },
        clearError(state) {
            state.error = null;
        }
    },
    extraReducers: (builder) => {
        builder
            // 处理获取帖子列表
            .addCase(fetchPosts.pending, (state) => {
                state.loading = true;
                state.error = null;
            })
            .addCase(fetchPosts.fulfilled, (state, action) => {
                state.loading = false;
                state.posts = action.payload;
            })
            .addCase(fetchPosts.rejected, (state, action) => {
                state.loading = false;
                state.error = action.payload as string;
            })

            // 处理获取帖子详情
            .addCase(fetchPostDetails.pending, (state) => {
                state.loading = true;
                state.error = null;
            })
            .addCase(fetchPostDetails.fulfilled, (state, action) => {
                state.loading = false;
                state.currentPost = action.payload;
            })
            .addCase(fetchPostDetails.rejected, (state, action) => {
                state.loading = false;
                state.error = action.payload as string;
            })

            // 处理创建帖子
            .addCase(createPost.pending, (state) => {
                state.loading = true;
                state.error = null;
            })
            .addCase(createPost.fulfilled, (state, action) => {
                state.loading = false;
                state.posts = action.payload;
            })
            .addCase(createPost.rejected, (state, action) => {
                state.loading = false;
                state.error = action.payload as string;
            });
    }
});

export const { clearCurrentPost, clearPosts, clearError } = postSlice.actions;
export default postSlice.reducer; 