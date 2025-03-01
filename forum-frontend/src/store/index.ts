import { configureStore } from '@reduxjs/toolkit';
import authReducer from './authSlice';
import postReducer from './postSlice';
import fieldReducer from './fieldSlice';

const store = configureStore({
    reducer: {
        auth: authReducer,
        posts: postReducer,
        fields: fieldReducer
    }
});

export type RootState = ReturnType<typeof store.getState>;
export type AppDispatch = typeof store.dispatch;

export default store; 