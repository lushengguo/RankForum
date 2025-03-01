import React, { useState } from 'react';
import styled from 'styled-components';
import { Link } from 'react-router-dom';
import { Post, Comment } from '../../types';
import { formatDateTime, formatScore, generateAvatarUrl, getDisplayName } from '../../utils/helpers';
import CommentList from '../comment/CommentList';
import Button from '../ui/Button';

interface PostDetailProps {
    post: Post;
    onUpvote?: (postId: string) => void;
    onDownvote?: (postId: string) => void;
    onComment?: (content: string, postId: string) => void;
    onCommentReply?: (content: string, commentId: string) => void;
    onUpvoteComment?: (commentId: string) => void;
    onDownvoteComment?: (commentId: string) => void;
}

const Container = styled.div`
  max-width: 900px;
  margin: 0 auto;
  padding: ${props => props.theme.spacing.large};
  background-color: ${props => props.theme.colors.background};
  border-radius: ${props => props.theme.borderRadius.large};
  box-shadow: ${props => props.theme.shadows.medium};
`;

const Header = styled.div`
  margin-bottom: ${props => props.theme.spacing.large};
`;

const Title = styled.h1`
  font-size: ${props => props.theme.fontSizes.xxlarge};
  font-weight: ${props => props.theme.typography.fontWeightBold};
  color: ${props => props.theme.colors.text};
  margin-bottom: ${props => props.theme.spacing.small};
  line-height: 1.3;
`;

const MetaContainer = styled.div`
  display: flex;
  align-items: center;
  margin-bottom: ${props => props.theme.spacing.medium};
  flex-wrap: wrap;
  gap: ${props => props.theme.spacing.small};
`;

const AuthorContainer = styled.div`
  display: flex;
  align-items: center;
  gap: ${props => props.theme.spacing.xsmall};
`;

const AuthorAvatar = styled.img`
  width: 32px;
  height: 32px;
  border-radius: 50%;
  object-fit: cover;
`;

const AuthorName = styled(Link)`
  font-weight: ${props => props.theme.typography.fontWeightMedium};
  color: ${props => props.theme.colors.primary};
  
  &:hover {
    text-decoration: underline;
  }
`;

const PostTime = styled.span`
  color: ${props => props.theme.colors.grey};
  font-size: ${props => props.theme.fontSizes.small};
`;

const Content = styled.div`
  font-size: ${props => props.theme.fontSizes.medium};
  line-height: 1.7;
  margin-bottom: ${props => props.theme.spacing.xlarge};
  color: ${props => props.theme.colors.text};
  white-space: pre-wrap;
  overflow-wrap: break-word;
`;

const VoteContainer = styled.div`
  display: flex;
  align-items: center;
  margin-bottom: ${props => props.theme.spacing.large};
  background-color: ${props => props.theme.colors.lightBackground};
  border-radius: ${props => props.theme.borderRadius.medium};
  padding: ${props => props.theme.spacing.small};
  width: fit-content;
`;

const Score = styled.span<{ isPositive: boolean }>`
  font-weight: ${props => props.theme.typography.fontWeightBold};
  font-size: ${props => props.theme.fontSizes.large};
  color: ${props => props.isPositive ? props.theme.colors.secondary : props.theme.colors.quaternary};
  margin: 0 ${props => props.theme.spacing.medium};
  min-width: 40px;
  text-align: center;
`;

const Divider = styled.hr`
  border: none;
  border-top: 1px solid ${props => props.theme.colors.lightGrey};
  margin: ${props => props.theme.spacing.large} 0;
`;

const Field = styled(Link)`
  display: inline-block;
  font-size: ${props => props.theme.fontSizes.small};
  color: ${props => props.theme.colors.white};
  background-color: ${props => props.theme.colors.tertiary};
  padding: 4px 12px;
  border-radius: 16px;
  text-decoration: none;
  
  &:hover {
    background-color: ${props => props.theme.colors.tertiaryDark};
  }
`;

const PostDetail: React.FC<PostDetailProps> = ({
    post,
    onUpvote,
    onDownvote,
    onComment,
    onCommentReply,
    onUpvoteComment,
    onDownvoteComment
}) => {
    const [authorName, setAuthorName] = useState<string | undefined>(undefined);
    const [fieldName, setFieldName] = useState<string | undefined>(undefined);

    // 在实际应用中，这里应该通过API获取用户名和领域名
    // 这里暂时使用地址前几位作为演示
    const handleUpvote = () => {
        if (onUpvote) {
            onUpvote(post.address);
        }
    };

    const handleDownvote = () => {
        if (onDownvote) {
            onDownvote(post.address);
        }
    };

    const handleAddComment = (content: string) => {
        if (onComment) {
            onComment(content, post.address);
        }
    };

    const handleCommentReply = (content: string, commentId: string) => {
        if (onCommentReply) {
            onCommentReply(content, commentId);
        }
    };

    const isPositiveScore = !post.score.startsWith('-');
    const displayName = getDisplayName(authorName, post.from);

    return (
        <Container>
            <Header>
                <Title>{post.title}</Title>

                <MetaContainer>
                    <AuthorContainer>
                        <AuthorAvatar
                            src={generateAvatarUrl(post.from)}
                            alt={displayName}
                        />
                        <AuthorName to={`/user/${post.from}`}>
                            {displayName}
                        </AuthorName>
                    </AuthorContainer>

                    <PostTime>{formatDateTime(post.timestamp)}</PostTime>

                    <Field to={`/field/${post.to}`}>
                        {fieldName || `领域 ${post.to.substring(0, 8)}...`}
                    </Field>
                </MetaContainer>
            </Header>

            <Content>{post.content}</Content>

            <VoteContainer>
                <Button
                    variant="outlined"
                    onClick={handleUpvote}
                    icon={<span>👍</span>}
                >
                    {post.upvote}
                </Button>

                <Score isPositive={isPositiveScore}>
                    {formatScore(post.score)}
                </Score>

                <Button
                    variant="outlined"
                    onClick={handleDownvote}
                    icon={<span>👎</span>}
                >
                    {post.downvote}
                </Button>
            </VoteContainer>

            <Divider />

            <CommentList
                comments={post.comments}
                postId={post.address}
                onAddComment={handleAddComment}
                onUpvote={onUpvoteComment}
                onDownvote={onDownvoteComment}
            />
        </Container>
    );
};

export default PostDetail; 