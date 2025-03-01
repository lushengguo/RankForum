import { createSlice, PayloadAction, createAsyncThunk } from '@reduxjs/toolkit';
import { Field } from '../types';
import { queryAPI } from '../services/api';

interface FieldState {
    fields: Field[];
    currentField: Field | null;
    loading: boolean;
    error: string | null;
}

const initialState: FieldState = {
    fields: [],
    currentField: null,
    loading: false,
    error: null
};

// 通过领域名称获取领域地址
export const fetchFieldAddress = createAsyncThunk(
    'fields/fetchAddress',
    async (field_name: string, { rejectWithValue }) => {
        try {
            const address = await queryAPI.getFieldAddress(field_name);
            return { name: field_name, address };
        } catch (error) {
            return rejectWithValue("获取领域地址失败");
        }
    }
);

const fieldSlice = createSlice({
    name: 'fields',
    initialState,
    reducers: {
        setCurrentField(state, action: PayloadAction<Field>) {
            state.currentField = action.payload;
        },
        addField(state, action: PayloadAction<Field>) {
            if (!state.fields.some(f => f.address === action.payload.address)) {
                state.fields.push(action.payload);
            }
        },
        clearFields(state) {
            state.fields = [];
        },
        clearCurrentField(state) {
            state.currentField = null;
        }
    },
    extraReducers: (builder) => {
        builder
            .addCase(fetchFieldAddress.pending, (state) => {
                state.loading = true;
                state.error = null;
            })
            .addCase(fetchFieldAddress.fulfilled, (state, action) => {
                state.loading = false;
                const newField = {
                    name: action.payload.name,
                    address: action.payload.address
                };
                // 更新当前领域
                state.currentField = newField;
                // 添加到领域列表，如果不存在
                if (!state.fields.some(f => f.address === newField.address)) {
                    state.fields.push(newField);
                }
            })
            .addCase(fetchFieldAddress.rejected, (state, action) => {
                state.loading = false;
                state.error = action.payload as string;
            });
    }
});

export const { setCurrentField, addField, clearFields, clearCurrentField } = fieldSlice.actions;
export default fieldSlice.reducer; 