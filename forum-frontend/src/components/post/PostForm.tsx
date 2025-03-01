import React, { useState, useEffect } from 'react';
import styled from 'styled-components';
import Button from '../ui/Button';
import Input from '../ui/Input';

interface Field {
    name: string;
    address: string;
}

interface PostFormProps {
    onSubmit: (title: string, content: string, fieldAddress: string) => void;
    onCancel?: () => void;
    availableFields?: Field[];
    initialTitle?: string;
    initialContent?: string;
    initialField?: string;
    isEditing?: boolean;
}

const FormContainer = styled.div`
  background-color: ${props => props.theme.colors.background};
  border-radius: ${props => props.theme.borderRadius.large};
  box-shadow: ${props => props.theme.shadows.medium};
  padding: ${props => props.theme.spacing.large};
  margin-bottom: ${props => props.theme.spacing.large};
`;

const FormTitle = styled.h2`
  font-size: ${props => props.theme.fontSizes.xlarge};
  margin-bottom: ${props => props.theme.spacing.medium};
  color: ${props => props.theme.colors.text};
`;

const FormGroup = styled.div`
  margin-bottom: ${props => props.theme.spacing.medium};
`;

const Label = styled.label`
  display: block;
  margin-bottom: ${props => props.theme.spacing.xsmall};
  font-weight: ${props => props.theme.typography.fontWeightMedium};
  color: ${props => props.theme.colors.darkGrey};
`;

const StyledTextarea = styled.textarea`
  width: 100%;
  min-height: 200px;
  padding: ${props => props.theme.spacing.medium};
  border: 1px solid ${props => props.theme.colors.lightGrey};
  border-radius: ${props => props.theme.borderRadius.medium};
  font-family: ${props => props.theme.typography.fontFamily};
  font-size: ${props => props.theme.fontSizes.medium};
  resize: vertical;
  
  &:focus {
    outline: none;
    border-color: ${props => props.theme.colors.primary};
    box-shadow: 0 0 0 2px ${props => props.theme.colors.primaryLight};
  }
`;

const CharacterCount = styled.div<{ isExceeded: boolean }>`
  text-align: right;
  margin-top: ${props => props.theme.spacing.xsmall};
  font-size: ${props => props.theme.fontSizes.small};
  color: ${props => props.isExceeded
        ? props.theme.colors.quaternary
        : props.theme.colors.grey};
`;

const Select = styled.select`
  width: 100%;
  padding: ${props => props.theme.spacing.medium};
  border: 1px solid ${props => props.theme.colors.lightGrey};
  border-radius: ${props => props.theme.borderRadius.medium};
  font-family: ${props => props.theme.typography.fontFamily};
  font-size: ${props => props.theme.fontSizes.medium};
  background-color: ${props => props.theme.colors.background};
  
  &:focus {
    outline: none;
    border-color: ${props => props.theme.colors.primary};
    box-shadow: 0 0 0 2px ${props => props.theme.colors.primaryLight};
  }
`;

const ButtonContainer = styled.div`
  display: flex;
  justify-content: flex-end;
  gap: ${props => props.theme.spacing.medium};
  margin-top: ${props => props.theme.spacing.large};
`;

const MAX_TITLE_LENGTH = 100;
const MAX_CONTENT_LENGTH = 10000;

const PostForm: React.FC<PostFormProps> = ({
    onSubmit,
    onCancel,
    availableFields = [],
    initialTitle = '',
    initialContent = '',
    initialField = '',
    isEditing = false
}) => {
    const [title, setTitle] = useState(initialTitle);
    const [content, setContent] = useState(initialContent);
    const [fieldAddress, setFieldAddress] = useState(initialField);
    const [titleError, setTitleError] = useState('');
    const [contentError, setContentError] = useState('');
    const [fieldError, setFieldError] = useState('');

    useEffect(() => {
        // 如果availableFields更新且当前选择的field为空，则自动选择第一个
        if (availableFields.length > 0 && !fieldAddress) {
            setFieldAddress(availableFields[0].address);
        }
    }, [availableFields]);

    const validateForm = (): boolean => {
        let isValid = true;

        if (!title.trim()) {
            setTitleError('标题不能为空');
            isValid = false;
        } else if (title.length > MAX_TITLE_LENGTH) {
            setTitleError(`标题不能超过${MAX_TITLE_LENGTH}个字符`);
            isValid = false;
        } else {
            setTitleError('');
        }

        if (!content.trim()) {
            setContentError('内容不能为空');
            isValid = false;
        } else if (content.length > MAX_CONTENT_LENGTH) {
            setContentError(`内容不能超过${MAX_CONTENT_LENGTH}个字符`);
            isValid = false;
        } else {
            setContentError('');
        }

        if (!fieldAddress) {
            setFieldError('请选择发布领域');
            isValid = false;
        } else {
            setFieldError('');
        }

        return isValid;
    };

    const handleSubmit = (e: React.FormEvent) => {
        e.preventDefault();

        if (validateForm()) {
            onSubmit(title, content, fieldAddress);
        }
    };

    const handleCancel = () => {
        if (onCancel) {
            onCancel();
        }
    };

    const titleCharsRemaining = MAX_TITLE_LENGTH - title.length;
    const contentCharsRemaining = MAX_CONTENT_LENGTH - content.length;

    return (
        <FormContainer>
            <FormTitle>{isEditing ? '编辑帖子' : '发布新帖子'}</FormTitle>

            <form onSubmit={handleSubmit}>
                <FormGroup>
                    <Label htmlFor="title">标题</Label>
                    <Input
                        id="title"
                        value={title}
                        onChange={(e) => setTitle(e.target.value)}
                        error={!!titleError}
                        helperText={titleError}
                        fullWidth
                    />
                    <CharacterCount isExceeded={titleCharsRemaining < 0}>
                        {titleCharsRemaining}
                    </CharacterCount>
                </FormGroup>

                <FormGroup>
                    <Label htmlFor="field">发布领域</Label>
                    <Select
                        id="field"
                        value={fieldAddress}
                        onChange={(e) => setFieldAddress(e.target.value)}
                    >
                        {availableFields.length === 0 ? (
                            <option value="">加载领域中...</option>
                        ) : (
                            availableFields.map((field) => (
                                <option key={field.address} value={field.address}>
                                    {field.name}
                                </option>
                            ))
                        )}
                    </Select>
                    {fieldError && (
                        <CharacterCount isExceeded={true}>
                            {fieldError}
                        </CharacterCount>
                    )}
                </FormGroup>

                <FormGroup>
                    <Label htmlFor="content">内容</Label>
                    <StyledTextarea
                        id="content"
                        value={content}
                        onChange={(e) => setContent(e.target.value)}
                    />
                    <CharacterCount isExceeded={contentCharsRemaining < 0}>
                        {contentCharsRemaining}
                    </CharacterCount>
                    {contentError && (
                        <CharacterCount isExceeded={true}>
                            {contentError}
                        </CharacterCount>
                    )}
                </FormGroup>

                <ButtonContainer>
                    {onCancel && (
                        <Button
                            type="button"
                            variant="outlined"
                            onClick={handleCancel}
                        >
                            取消
                        </Button>
                    )}

                    <Button
                        type="submit"
                        variant="primary"
                        disabled={!title.trim() || !content.trim() || !fieldAddress}
                    >
                        {isEditing ? '保存修改' : '发布帖子'}
                    </Button>
                </ButtonContainer>
            </form>
        </FormContainer>
    );
};

export default PostForm; 