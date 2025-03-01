import React from 'react';
import styled from 'styled-components';
import { Link } from 'react-router-dom';

interface Field {
    name: string;
    address: string;
    description?: string;
    postCount?: number;
}

interface FieldListProps {
    fields: Field[];
    isLoading?: boolean;
}

const Container = styled.div`
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
  gap: ${props => props.theme.spacing.medium};
  margin-bottom: ${props => props.theme.spacing.xlarge};
  
  @media (max-width: ${props => props.theme.breakpoints.md}) {
    grid-template-columns: 1fr;
  }
`;

const FieldCard = styled(Link)`
  text-decoration: none;
  color: inherit;
  background-color: ${props => props.theme.colors.background};
  border-radius: ${props => props.theme.borderRadius.medium};
  box-shadow: ${props => props.theme.shadows.small};
  padding: ${props => props.theme.spacing.medium};
  transition: ${props => props.theme.transitions.default};
  border: 1px solid ${props => props.theme.colors.lightGrey};
  display: flex;
  flex-direction: column;
  height: 100%;
  
  &:hover {
    transform: translateY(-4px);
    box-shadow: ${props => props.theme.shadows.medium};
    border-color: ${props => props.theme.colors.primary};
  }
`;

const FieldName = styled.h3`
  font-size: ${props => props.theme.fontSizes.large};
  font-weight: ${props => props.theme.typography.fontWeightBold};
  color: ${props => props.theme.colors.text};
  margin-bottom: ${props => props.theme.spacing.small};
`;

const FieldDescription = styled.p`
  font-size: ${props => props.theme.fontSizes.medium};
  color: ${props => props.theme.colors.grey};
  margin-bottom: ${props => props.theme.spacing.medium};
  flex-grow: 1;
`;

const FieldMeta = styled.div`
  display: flex;
  justify-content: space-between;
  align-items: center;
  font-size: ${props => props.theme.fontSizes.small};
  color: ${props => props.theme.colors.darkGrey};
`;

const PostCount = styled.span`
  background-color: ${props => props.theme.colors.lightBackground};
  padding: 4px 8px;
  border-radius: ${props => props.theme.borderRadius.small};
`;

const LoadingContainer = styled.div`
  padding: ${props => props.theme.spacing.xlarge};
  text-align: center;
  color: ${props => props.theme.colors.grey};
`;

const EmptyState = styled.div`
  padding: ${props => props.theme.spacing.xlarge};
  text-align: center;
  color: ${props => props.theme.colors.grey};
  border: 1px dashed ${props => props.theme.colors.lightGrey};
  border-radius: ${props => props.theme.borderRadius.medium};
`;

const FieldList: React.FC<FieldListProps> = ({ fields, isLoading = false }) => {
    if (isLoading) {
        return (
            <LoadingContainer>
                <h3>正在加载领域列表...</h3>
            </LoadingContainer>
        );
    }

    if (fields.length === 0) {
        return (
            <EmptyState>
                <h3>暂无可用领域</h3>
                <p>等待管理员创建新的论坛领域</p>
            </EmptyState>
        );
    }

    return (
        <Container>
            {fields.map(field => (
                <FieldCard key={field.address} to={`/field/${field.address}`}>
                    <FieldName>{field.name}</FieldName>
                    <FieldDescription>
                        {field.description || '暂无描述信息'}
                    </FieldDescription>
                    <FieldMeta>
                        <span>地址: {field.address.substring(0, 8)}...</span>
                        {field.postCount !== undefined && (
                            <PostCount>{field.postCount} 帖子</PostCount>
                        )}
                    </FieldMeta>
                </FieldCard>
            ))}
        </Container>
    );
};

export default FieldList; 