import React from 'react';
import styled from 'styled-components';
import { Link } from 'react-router-dom';
import { Post } from '../../types';
import { formatScore, getRelativeTime, truncateText } from '../../utils/helpers';
import Button from '../ui/Button';

interface PostCardProps {
    post: Post;
    onUpvote?: (postId: string) => void;
    onDownvote?: (postId: string) => void;
}

const Card = styled.div`
  background-color: ${props => props.theme.colors.cardBackground};
  border-radius: ${props => props.theme.borderRadius.medium};
  box-shadow: ${props => props.theme.shadows.small};
  padding: ${props => props.theme.spacing.medium};
  margin-bottom: ${props => props.theme.spacing.medium};
  transition: all ${props => props.theme.transitions.normal};
  border: 1px solid ${props => props.theme.colors.lightGrey};
  
  &:hover {
    box-shadow: ${props => props.theme.shadows.medium};
    transform: translateY(-2px);
  }
`;

const Title = styled.h3`
  color: ${props => props.theme.colors.text};
  margin-bottom: ${props => props.theme.spacing.xsmall};
  font-weight: ${props => props.theme.typography.fontWeightMedium};
  
  a {
    color: inherit;
    text-decoration: none;
    
    &:hover {
      color: ${props => props.theme.colors.primary};
      text-decoration: none;
    }
  }
`;

const Content = styled.p`
  color: ${props => props.theme.colors.grey};
  font-size: ${props => props.theme.fontSizes.medium};
  margin-bottom: ${props => props.theme.spacing.medium};
  line-height: 1.6;
`;

const MetaData = styled.div`
  display: flex;
  align-items: center;
  justify-content: space-between;
  font-size: ${props => props.theme.fontSizes.small};
  color: ${props => props.theme.colors.grey};
  margin-bottom: ${props => props.theme.spacing.medium};
`;

const PostInfo = styled.div`
  display: flex;
  align-items: center;
  gap: ${props => props.theme.spacing.small};
`;

const Author = styled(Link)`
  font-weight: ${props => props.theme.typography.fontWeightMedium};
  color: ${props => props.theme.colors.grey};
  
  &:hover {
    color: ${props => props.theme.colors.primary};
  }
`;

const Timestamp = styled.span`
  color: ${props => props.theme.colors.grey};
`;

const VoteContainer = styled.div`
  display: flex;
  align-items: center;
  gap: ${props => props.theme.spacing.xxsmall};
  border-top: 1px solid ${props => props.theme.colors.lightGrey};
  padding-top: ${props => props.theme.spacing.small};
`;

const Score = styled.span<{ isPositive: boolean }>`
  font-weight: ${props => props.theme.typography.fontWeightBold};
  color: ${props => props.isPositive ? props.theme.colors.secondary : props.theme.colors.quaternary};
  min-width: 40px;
  text-align: center;
`;

const PostCard: React.FC<PostCardProps> = ({ post, onUpvote, onDownvote }) => {
    const isPositiveScore = !post.score.startsWith('-');

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

    return (
        <Card>
            <MetaData>
                <PostInfo>
                    <Author to={`/user/${post.from}`}>匿名用户</Author>
                    <Timestamp>{getRelativeTime(post.timestamp)}</Timestamp>
                </PostInfo>
            </MetaData>

            <Title>
                <Link to={`/post/${post.address}`}>{post.title}</Link>
            </Title>

            <Content>{truncateText(post.content, 150)}</Content>

            <VoteContainer>
                <Button
                    variant="text"
                    size="small"
                    onClick={handleUpvote}
                    icon={<span>👍</span>}
                >
                    {post.upvote}
                </Button>

                <Score isPositive={isPositiveScore}>{formatScore(post.score)}</Score>

                <Button
                    variant="text"
                    size="small"
                    onClick={handleDownvote}
                    icon={<span>👎</span>}
                >
                    {post.downvote}
                </Button>

                <div style={{ flex: 1 }} />

                <Button
                    variant="text"
                    size="small"
                    icon={<span>💬</span>}
                    onClick={() => { }}
                >
                    {post.comments.length}
                </Button>
            </VoteContainer>
        </Card>
    );
};

export default PostCard; 