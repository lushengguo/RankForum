import React, { useState } from 'react';
import styled from 'styled-components';
import { Comment } from '../../types';
import CommentItem from './CommentItem';
import CommentForm from './CommentForm';

interface CommentListProps {
    comments: Comment[];
    postId?: string;
    onAddComment?: (content: string, parentId?: string) => void;
    onUpvote?: (commentId: string) => void;
    onDownvote?: (commentId: string) => void;
}

const Container = styled.div`
  margin-top: ${props => props.theme.spacing.large};
`;

const CommentHeader = styled.h3`
  font-size: ${props => props.theme.fontSizes.large};
  font-weight: ${props => props.theme.typography.fontWeightBold};
  margin-bottom: ${props => props.theme.spacing.medium};
`;

const NoComments = styled.p`
  color: ${props => props.theme.colors.grey};
  font-style: italic;
  padding: ${props => props.theme.spacing.medium};
  text-align: center;
  border: 1px dashed ${props => props.theme.colors.lightGrey};
  border-radius: ${props => props.theme.borderRadius.medium};
`;

const ReplyFormContainer = styled.div`
  margin-top: ${props => props.theme.spacing.medium};
  margin-bottom: ${props => props.theme.spacing.large};
  padding: ${props => props.theme.spacing.medium};
  border-radius: ${props => props.theme.borderRadius.medium};
  background-color: ${props => props.theme.colors.lightBackground};
`;

const ReplyHeader = styled.h4`
  margin-bottom: ${props => props.theme.spacing.small};
  font-size: ${props => props.theme.fontSizes.medium};
  color: ${props => props.theme.colors.darkGrey};
`;

const CommentList: React.FC<CommentListProps> = ({
    comments,
    postId,
    onAddComment,
    onUpvote,
    onDownvote
}) => {
    const [replyingTo, setReplyingTo] = useState<string | null>(null);

    const handleReply = (commentId: string) => {
        setReplyingTo(commentId);
    };

    const handleSubmitReply = (content: string) => {
        if (onAddComment && replyingTo) {
            onAddComment(content, replyingTo);
            setReplyingTo(null);
        }
    };

    const handleCancelReply = () => {
        setReplyingTo(null);
    };

    return (
        <Container>
            <CommentHeader>评论 ({comments.length})</CommentHeader>

            {postId && onAddComment && (
                <CommentForm
                    onSubmit={(content) => onAddComment(content)}
                    placeholder="发表你的看法..."
                    submitLabel="发表评论"
                />
            )}

            {replyingTo && (
                <ReplyFormContainer>
                    <ReplyHeader>回复评论</ReplyHeader>
                    <CommentForm
                        onSubmit={handleSubmitReply}
                        onCancel={handleCancelReply}
                        placeholder="写下你的回复..."
                        submitLabel="回复"
                        showCancel
                    />
                </ReplyFormContainer>
            )}

            {comments.length === 0 ? (
                <NoComments>暂无评论，成为第一个评论的人吧！</NoComments>
            ) : (
                comments.map(comment => (
                    <CommentItem
                        key={comment.address}
                        comment={comment}
                        onReply={handleReply}
                        onUpvote={onUpvote}
                        onDownvote={onDownvote}
                    />
                ))
            )}
        </Container>
    );
};

export default CommentList; 