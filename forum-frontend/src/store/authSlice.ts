import { createSlice, PayloadAction, createAsyncThunk } from '@reduxjs/toolkit';
import { AuthState, User } from '../types';
import { authAPI } from '../services/api';

const initialState: AuthState = {
    isAuthenticated: false,
    user: null,
    sessionId: null
};

// 异步登录操作
export const loginUser = createAsyncThunk(
    'auth/login',
    async ({ pubkey, signed_pubkey }: { pubkey: string, signed_pubkey: string }, { rejectWithValue }) => {
        try {
            const sid = await authAPI.login(pubkey, signed_pubkey);
            localStorage.setItem('sid', sid);
            return {
                address: pubkey,
                sid
            };
        } catch (error) {
            return rejectWithValue("登录失败，请检查您的凭据");
        }
    }
);

// 异步创建用户操作
export const createUser = createAsyncThunk(
    'auth/createUser',
    async (user_name: string, { getState, rejectWithValue }) => {
        try {
            await authAPI.createUser(user_name);
            const { auth } = getState() as { auth: AuthState };
            if (auth.user?.address) {
                return {
                    ...auth.user,
                    name: user_name
                };
            }
            return rejectWithValue("未找到用户地址");
        } catch (error) {
            return rejectWithValue("创建用户失败");
        }
    }
);

// 异步重命名用户操作
export const renameUser = createAsyncThunk(
    'auth/renameUser',
    async ({ name, address }: { name: string, address: string }, { rejectWithValue }) => {
        try {
            await authAPI.renameUser(name, address);
            return { name, address };
        } catch (error) {
            return rejectWithValue("重命名用户失败");
        }
    }
);

const authSlice = createSlice({
    name: 'auth',
    initialState,
    reducers: {
        logout(state) {
            state.isAuthenticated = false;
            state.user = null;
            state.sessionId = null;
            localStorage.removeItem('sid');
        },
        setUser(state, action: PayloadAction<User>) {
            state.user = action.payload;
        },
        restoreSession(state, action: PayloadAction<string>) {
            state.sessionId = action.payload;
            state.isAuthenticated = true;
        }
    },
    extraReducers: (builder) => {
        builder
            .addCase(loginUser.fulfilled, (state, action) => {
                state.isAuthenticated = true;
                state.sessionId = action.payload.sid;
                state.user = {
                    address: action.payload.address,
                    name: '' // 初始时名称为空，需要用户设置
                };
            })
            .addCase(createUser.fulfilled, (state, action) => {
                if (state.user) {
                    state.user.name = action.payload.name;
                }
            })
            .addCase(renameUser.fulfilled, (state, action) => {
                if (state.user && state.user.address === action.payload.address) {
                    state.user.name = action.payload.name;
                }
            });
    }
});

export const { logout, setUser, restoreSession } = authSlice.actions;
export default authSlice.reducer; 