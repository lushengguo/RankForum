import React, { useState } from 'react';
import styled from 'styled-components';
import { Link } from 'react-router-dom';
import { Comment } from '../../types';
import { formatScore, getRelativeTime } from '../../utils/helpers';
import Button from '../ui/Button';

interface CommentItemProps {
    comment: Comment;
    onReply?: (commentId: string) => void;
    onUpvote?: (commentId: string) => void;
    onDownvote?: (commentId: string) => void;
    depth?: number;
}

const CommentContainer = styled.div<{ depth: number }>`
  margin-left: ${props => props.depth > 0 ? `${props.depth * 24}px` : '0'};
  margin-bottom: ${props => props.theme.spacing.medium};
  padding: ${props => props.theme.spacing.medium};
  background-color: ${props => props.theme.colors.background};
  border-radius: ${props => props.theme.borderRadius.medium};
  border: 1px solid ${props => props.theme.colors.lightGrey};
`;

const CommentHeader = styled.div`
  display: flex;
  align-items: center;
  margin-bottom: ${props => props.theme.spacing.small};
  gap: ${props => props.theme.spacing.small};
`;

const Author = styled(Link)`
  font-weight: ${props => props.theme.typography.fontWeightMedium};
  color: ${props => props.theme.colors.darkGrey};
  font-size: ${props => props.theme.fontSizes.small};
  
  &:hover {
    color: ${props => props.theme.colors.primary};
  }
`;

const Timestamp = styled.span`
  color: ${props => props.theme.colors.grey};
  font-size: ${props => props.theme.fontSizes.small};
`;

const CommentContent = styled.div`
  margin-bottom: ${props => props.theme.spacing.small};
  font-size: ${props => props.theme.fontSizes.medium};
  line-height: 1.6;
  color: ${props => props.theme.colors.text};
`;

const ActionBar = styled.div`
  display: flex;
  align-items: center;
  gap: ${props => props.theme.spacing.xsmall};
`;

const Score = styled.span<{ isPositive: boolean }>`
  font-weight: ${props => props.theme.typography.fontWeightBold};
  color: ${props => props.isPositive ? props.theme.colors.secondary : props.theme.colors.quaternary};
  min-width: 30px;
  text-align: center;
  font-size: ${props => props.theme.fontSizes.small};
`;

const ChildComments = styled.div`
  margin-top: ${props => props.theme.spacing.medium};
`;

const CommentItem: React.FC<CommentItemProps> = ({
    comment,
    onReply,
    onUpvote,
    onDownvote,
    depth = 0
}) => {
    const [showChildComments, setShowChildComments] = useState(true);
    const isPositiveScore = !comment.score.startsWith('-');

    const handleUpvote = () => {
        if (onUpvote) {
            onUpvote(comment.address);
        }
    };

    const handleDownvote = () => {
        if (onDownvote) {
            onDownvote(comment.address);
        }
    };

    const handleReply = () => {
        if (onReply) {
            onReply(comment.address);
        }
    };

    const toggleChildComments = () => {
        setShowChildComments(!showChildComments);
    };

    return (
        <>
            <CommentContainer depth={depth}>
                <CommentHeader>
                    <Author to={`/user/${comment.from}`}>匿名用户</Author>
                    <Timestamp>{getRelativeTime(comment.timestamp)}</Timestamp>
                </CommentHeader>

                <CommentContent>{comment.content}</CommentContent>

                <ActionBar>
                    <Button
                        variant="text"
                        size="small"
                        onClick={handleUpvote}
                        icon={<span>👍</span>}
                    >
                        {comment.upvote}
                    </Button>

                    <Score isPositive={isPositiveScore}>{formatScore(comment.score)}</Score>

                    <Button
                        variant="text"
                        size="small"
                        onClick={handleDownvote}
                        icon={<span>👎</span>}
                    >
                        {comment.downvote}
                    </Button>

                    <div style={{ flex: 1 }} />

                    <Button
                        variant="text"
                        size="small"
                        onClick={handleReply}
                    >
                        回复
                    </Button>

                    {comment.comments.length > 0 && (
                        <Button
                            variant="text"
                            size="small"
                            onClick={toggleChildComments}
                        >
                            {showChildComments ? '收起回复' : `显示${comment.comments.length}条回复`}
                        </Button>
                    )}
                </ActionBar>
            </CommentContainer>

            {showChildComments && comment.comments.length > 0 && (
                <ChildComments>
                    {comment.comments.map(childComment => (
                        <CommentItem
                            key={childComment.address}
                            comment={childComment}
                            onReply={onReply}
                            onUpvote={onUpvote}
                            onDownvote={onDownvote}
                            depth={depth + 1}
                        />
                    ))}
                </ChildComments>
            )}
        </>
    );
};

export default CommentItem; 