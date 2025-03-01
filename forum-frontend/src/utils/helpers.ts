import { Comment, Post } from "../types";

// 格式化时间戳为可读格式
export const formatDate = (timestamp: number): string => {
    const date = new Date(timestamp * 1000);
    return date.toLocaleString('zh-CN', {
        year: 'numeric',
        month: '2-digit',
        day: '2-digit',
        hour: '2-digit',
        minute: '2-digit'
    });
};

// 截断长文本并添加省略号
export const truncateText = (text: string, maxLength: number): string => {
    if (text.length <= maxLength) return text;
    return text.slice(0, maxLength) + '...';
};

/**
 * 格式化分数显示
 * @param score 分数字符串
 * @returns 格式化后的分数
 */
export const formatScore = (score: string): string => {
    const numScore = parseFloat(score);

    if (isNaN(numScore)) {
        return '0';
    }

    if (Math.abs(numScore) >= 1000000) {
        return (numScore / 1000000).toFixed(1) + 'M';
    }

    if (Math.abs(numScore) >= 1000) {
        return (numScore / 1000).toFixed(1) + 'K';
    }

    return numScore.toString();
};

// 获取用户名或显示地址的前几位
export const getDisplayName = (name: string | undefined, address: string): string => {
    if (name && name.trim() !== "") return name;
    return address.substring(0, 8) + "...";
};

// 根据得分计算帖子热度
export const calculateHeat = (post: Post): number => {
    const score = parseInt(post.score, 10) || 0;
    const upvotes = post.upvote || 0;
    const downvotes = post.downvote || 0;
    const commentCount = post.comments.length;

    // 简单计算热度，可根据需要调整
    return score + upvotes * 2 - downvotes + commentCount * 3;
};

// 递归计算评论总数
export const countAllComments = (comments: Comment[]): number => {
    let count = comments.length;
    for (const comment of comments) {
        if (comment.comments && comment.comments.length > 0) {
            count += countAllComments(comment.comments);
        }
    }
    return count;
};

// 根据时间戳计算相对时间（例如：3小时前，2天前）
export const getRelativeTime = (timestamp: number): string => {
    const now = Math.floor(Date.now() / 1000);
    const diff = now - timestamp;

    if (diff < 60) return "刚刚";
    if (diff < 3600) return `${Math.floor(diff / 60)}分钟前`;
    if (diff < 86400) return `${Math.floor(diff / 3600)}小时前`;
    if (diff < 2592000) return `${Math.floor(diff / 86400)}天前`;
    if (diff < 31536000) return `${Math.floor(diff / 2592000)}个月前`;
    return `${Math.floor(diff / 31536000)}年前`;
};

/**
 * 从用户地址或用户名生成头像URL
 * 使用开源的头像服务生成一个基于字符串的头像
 * @param identifier 用户标识（地址或用户名）
 * @returns 头像URL
 */
export const generateAvatarUrl = (identifier: string): string => {
    // 使用简单的哈希算法，使相同的标识符生成相同的头像
    let hash = 0;
    for (let i = 0; i < identifier.length; i++) {
        hash = ((hash << 5) - hash) + identifier.charCodeAt(i);
        hash |= 0; // 转换为32位整数
    }

    // 确保hash是正数
    hash = Math.abs(hash);

    // 使用dicebear的头像服务
    return `https://api.dicebear.com/7.x/identicon/svg?seed=${hash}`;
};

/**
 * 格式化日期时间
 * @param timestamp 时间戳（秒）
 * @returns 格式化的日期时间字符串
 */
export const formatDateTime = (timestamp: number): string => {
    const date = new Date(timestamp * 1000);

    const year = date.getFullYear();
    const month = String(date.getMonth() + 1).padStart(2, '0');
    const day = String(date.getDate()).padStart(2, '0');
    const hours = String(date.getHours()).padStart(2, '0');
    const minutes = String(date.getMinutes()).padStart(2, '0');

    return `${year}-${month}-${day} ${hours}:${minutes}`;
}; 