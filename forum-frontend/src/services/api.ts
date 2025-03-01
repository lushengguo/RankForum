import axios from "axios";
import { Comment, Field, FilterOption, Post, User } from "../types";

// 创建一个axios实例
const api = axios.create({
    baseURL: process.env.REACT_APP_API_URL || "http://localhost:8000",
    headers: {
        "Content-Type": "application/json",
    },
});

// 添加请求拦截器，添加SID到请求中
api.interceptors.request.use((config) => {
    const sid = localStorage.getItem("sid");
    if (sid) {
        // 对于GET请求，添加到URL参数
        if (config.method?.toLowerCase() === "get") {
            config.params = { ...config.params, SID: sid };
        }
        // 对于POST请求，添加到请求体
        else if (config.method?.toLowerCase() === "post") {
            if (config.url !== "/login") {
                config.params = { ...config.params, SID: sid };
            }
        }
    }
    return config;
});

// 认证相关API
export const authAPI = {
    login: async (pubkey: string, signed_pubkey: string) => {
        const response = await api.post("/login", { pubkey, signed_pubkey });
        // 从响应文本中提取SID，格式为"login successful, SID=xxx"
        const text = response.data;
        const match = text.match(/SID=([a-zA-Z0-9]+)/);
        if (match && match[1]) {
            return match[1];
        }
        throw new Error("未能从响应中获取SID");
    },
    createUser: async (user_name: string) => {
        return await api.post("/create_user", { user_name });
    },
    renameUser: async (name: string, address: string) => {
        return await api.post("/rename_user", null, { params: { name, address } });
    }
};

// 帖子相关API
export const postAPI = {
    createPost: async (field_name: string, field_address: string, title: string, content: string) => {
        return await api.post("/post", null, {
            params: { field_name, field_address, title, content }
        });
    },
    filterPosts: async (field_name?: string, field_address?: string, options?: Partial<FilterOption>) => {
        const defaultOptions: FilterOption = {
            ordering: options?.ordering || "timestamp" as any,
            ascending: options?.ascending || false,
            max_results: options?.max_results || 10
        };

        const params = {
            field_name,
            field_address,
            ...defaultOptions,
            ...options
        };

        const response = await api.get("/filter_post", { params });
        return response.data as Post[];
    },
    getPostDetails: async (address: string) => {
        // 由于后端没有直接提供获取单个帖子的接口，这里可以通过帖子地址过滤实现
        const response = await api.get("/filter_post", {
            params: { post_address: address }
        });
        if (Array.isArray(response.data) && response.data.length > 0) {
            return response.data[0] as Post;
        }
        throw new Error("未找到帖子");
    }
};

// 评论相关API
export const commentAPI = {
    createComment: async (to: string, content: string, field_address: string) => {
        return await api.post("/comment", null, {
            params: { to, content, field_address }
        });
    }
};

// 投票相关API
export const voteAPI = {
    upvote: async (target_address: string, field_address: string) => {
        return await api.post("/upvote", null, {
            params: { target_address, field_address }
        });
    },
    downvote: async (target_address: string, field_address: string) => {
        return await api.post("/downvote", null, {
            params: { target_address, field_address }
        });
    }
};

// 添加一个用于获取所有字段的API方法
export const fieldAPI = {
    getAllFields: async () => {
        try {
            const response = await api.get("/get_all_fields");
            return response.data as Field[];
        } catch (error) {
            console.error("获取所有字段失败:", error);
            throw error;
        }
    },
    createField: async (field_name: string) => {
        try {
            const response = await api.post("/create_field", null, {
                params: { field_name }
            });
            return response.data;
        } catch (error) {
            console.error("创建字段失败:", error);
            throw error;
        }
    },
    getFieldPosts: async (field_name?: string, field_address?: string) => {
        try {
            const response = await api.get("/get_field_posts", {
                params: { field_name, field_address }
            });
            return response.data as Post[];
        } catch (error) {
            console.error("获取字段帖子失败:", error);
            throw error;
        }
    }
};

// 添加用于获取用户信息和帖子的API方法
export const userAPI = {
    getCurrentUser: async () => {
        try {
            const response = await api.get("/user_info");
            return response.data as User;
        } catch (error) {
            console.error("获取当前用户信息失败:", error);
            throw error;
        }
    },
    getUserPosts: async (user_address?: string) => {
        try {
            const response = await api.get("/user_posts", {
                params: { user_address }
            });
            return response.data as Post[];
        } catch (error) {
            console.error("获取用户帖子失败:", error);
            throw error;
        }
    }
};

// 查询相关API
export const queryAPI = {
    getUserAddress: async (user_name: string) => {
        const response = await api.get("/query_user_address", {
            params: { user_name }
        });
        return response.data as string;
    },
    getFieldAddress: async (field_name: string) => {
        const response = await api.get("/query_field_address", {
            params: { field_name }
        });
        return response.data as string;
    },
    getScoreInField: async (user_name: string, field_name: string, field_address?: string) => {
        const response = await api.get("/query_score_in_field", {
            params: { user_name, field_name, field_address }
        });
        return response.data as string;
    }
};

export default api; 