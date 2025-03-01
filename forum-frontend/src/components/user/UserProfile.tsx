import React from 'react';
import styled from 'styled-components';
import { formatDateTime, formatScore, generateAvatarUrl, getDisplayName } from '../../utils/helpers';
import Button from '../ui/Button';
import { Post, User } from '../../types';
import PostCard from '../post/PostCard';

interface UserProfileProps {
    user: User;
    userPosts: Post[];
    userScores: { field: string; fieldName: string; score: string }[];
    isCurrentUser?: boolean;
    isLoading?: boolean;
    onEditProfile?: () => void;
}

const Container = styled.div`
  max-width: 900px;
  margin: 0 auto;
  padding: ${props => props.theme.spacing.large};
`;

const ProfileHeader = styled.div`
  display: flex;
  align-items: flex-start;
  margin-bottom: ${props => props.theme.spacing.xlarge};
  
  @media (max-width: ${props => props.theme.breakpoints.md}) {
    flex-direction: column;
    align-items: center;
  }
`;

const AvatarContainer = styled.div`
  margin-right: ${props => props.theme.spacing.large};
  
  @media (max-width: ${props => props.theme.breakpoints.md}) {
    margin-right: 0;
    margin-bottom: ${props => props.theme.spacing.medium};
  }
`;

const Avatar = styled.img`
  width: 120px;
  height: 120px;
  border-radius: 50%;
  object-fit: cover;
  border: 4px solid ${props => props.theme.colors.primary};
`;

const ProfileInfo = styled.div`
  flex: 1;
`;

const Username = styled.h1`
  font-size: ${props => props.theme.fontSizes.xxlarge};
  font-weight: ${props => props.theme.typography.fontWeightBold};
  margin-bottom: ${props => props.theme.spacing.xsmall};
`;

const UserAddress = styled.div`
  font-family: monospace;
  color: ${props => props.theme.colors.grey};
  margin-bottom: ${props => props.theme.spacing.medium};
  word-break: break-all;
`;

const ActionButton = styled(Button)`
  margin-top: ${props => props.theme.spacing.small};
`;

const SectionTitle = styled.h2`
  font-size: ${props => props.theme.fontSizes.xlarge};
  font-weight: ${props => props.theme.typography.fontWeightBold};
  margin-bottom: ${props => props.theme.spacing.medium};
  margin-top: ${props => props.theme.spacing.xlarge};
`;

const ScoreContainer = styled.div`
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(220px, 1fr));
  gap: ${props => props.theme.spacing.medium};
  margin-bottom: ${props => props.theme.spacing.large};
`;

const ScoreCard = styled.div`
  padding: ${props => props.theme.spacing.medium};
  background-color: ${props => props.theme.colors.background};
  border-radius: ${props => props.theme.borderRadius.medium};
  box-shadow: ${props => props.theme.shadows.small};
  border: 1px solid ${props => props.theme.colors.lightGrey};
`;

const FieldName = styled.div`
  font-weight: ${props => props.theme.typography.fontWeightMedium};
  font-size: ${props => props.theme.fontSizes.medium};
  margin-bottom: ${props => props.theme.spacing.xsmall};
`;

const Score = styled.div<{ isPositive: boolean }>`
  font-size: ${props => props.theme.fontSizes.large};
  font-weight: ${props => props.theme.typography.fontWeightBold};
  color: ${props => props.isPositive
        ? props.theme.colors.secondary
        : props.theme.colors.quaternary};
`;

const PostsContainer = styled.div`
  display: flex;
  flex-direction: column;
  gap: ${props => props.theme.spacing.medium};
  margin-bottom: ${props => props.theme.spacing.xlarge};
`;

const NoContent = styled.div`
  padding: ${props => props.theme.spacing.large};
  text-align: center;
  color: ${props => props.theme.colors.grey};
  border: 1px dashed ${props => props.theme.colors.lightGrey};
  border-radius: ${props => props.theme.borderRadius.medium};
`;

const LoadingContainer = styled.div`
  padding: ${props => props.theme.spacing.xlarge};
  text-align: center;
  color: ${props => props.theme.colors.grey};
`;

const UserProfile: React.FC<UserProfileProps> = ({
    user,
    userPosts,
    userScores,
    isCurrentUser = false,
    isLoading = false,
    onEditProfile
}) => {
    if (isLoading) {
        return (
            <LoadingContainer>
                <h2>加载用户信息中...</h2>
            </LoadingContainer>
        );
    }

    const displayName = getDisplayName(user.name, user.address);

    return (
        <Container>
            <ProfileHeader>
                <AvatarContainer>
                    <Avatar src={generateAvatarUrl(user.address)} alt={displayName} />
                </AvatarContainer>

                <ProfileInfo>
                    <Username>{displayName}</Username>
                    <UserAddress>{user.address}</UserAddress>

                    {isCurrentUser && onEditProfile && (
                        <ActionButton
                            variant="outlined"
                            onClick={onEditProfile}
                        >
                            编辑个人资料
                        </ActionButton>
                    )}
                </ProfileInfo>
            </ProfileHeader>

            <SectionTitle>领域声誉</SectionTitle>
            {userScores.length === 0 ? (
                <NoContent>
                    <p>该用户尚未在任何领域获得声誉</p>
                </NoContent>
            ) : (
                <ScoreContainer>
                    {userScores.map((score) => (
                        <ScoreCard key={score.field}>
                            <FieldName>{score.fieldName || score.field.substring(0, 8) + '...'}</FieldName>
                            <Score isPositive={!score.score.startsWith('-')}>
                                {formatScore(score.score)}
                            </Score>
                        </ScoreCard>
                    ))}
                </ScoreContainer>
            )}

            <SectionTitle>发布的帖子</SectionTitle>
            {userPosts.length === 0 ? (
                <NoContent>
                    <p>该用户尚未发布任何帖子</p>
                </NoContent>
            ) : (
                <PostsContainer>
                    {userPosts.map((post) => (
                        <PostCard key={post.address} post={post} />
                    ))}
                </PostsContainer>
            )}
        </Container>
    );
};

export default UserProfile; 