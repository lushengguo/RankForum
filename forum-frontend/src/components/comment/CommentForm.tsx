import React, { useState } from 'react';
import styled from 'styled-components';
import Button from '../ui/Button';

interface CommentFormProps {
    onSubmit: (content: string) => void;
    onCancel?: () => void;
    placeholder?: string;
    submitLabel?: string;
    showCancel?: boolean;
    initialValue?: string;
}

const FormContainer = styled.form`
  margin-bottom: ${props => props.theme.spacing.medium};
`;

const TextareaWrapper = styled.div`
  position: relative;
  margin-bottom: ${props => props.theme.spacing.small};
`;

const CommentTextarea = styled.textarea`
  width: 100%;
  padding: ${props => props.theme.spacing.medium};
  border: 1px solid ${props => props.theme.colors.lightGrey};
  border-radius: ${props => props.theme.borderRadius.medium};
  font-family: ${props => props.theme.typography.fontFamily};
  font-size: ${props => props.theme.fontSizes.medium};
  resize: vertical;
  min-height: 100px;
  transition: border-color 0.2s ease;
  
  &:focus {
    outline: none;
    border-color: ${props => props.theme.colors.primary};
    box-shadow: 0 0 0 2px ${props => props.theme.colors.primaryLight};
  }
  
  &::placeholder {
    color: ${props => props.theme.colors.grey};
  }
`;

const CharacterCount = styled.div<{ isExceeded: boolean }>`
  position: absolute;
  bottom: 8px;
  right: 12px;
  font-size: ${props => props.theme.fontSizes.small};
  color: ${props => props.isExceeded
        ? props.theme.colors.quaternary
        : props.theme.colors.grey};
`;

const ButtonContainer = styled.div`
  display: flex;
  justify-content: flex-end;
  gap: ${props => props.theme.spacing.small};
`;

const MAX_COMMENT_LENGTH = 1000;

const CommentForm: React.FC<CommentFormProps> = ({
    onSubmit,
    onCancel,
    placeholder = '写下你的评论...',
    submitLabel = '提交',
    showCancel = false,
    initialValue = ''
}) => {
    const [content, setContent] = useState(initialValue);

    const handleSubmit = (e: React.FormEvent) => {
        e.preventDefault();

        if (content.trim() && content.length <= MAX_COMMENT_LENGTH) {
            onSubmit(content);
            setContent('');
        }
    };

    const handleChange = (e: React.ChangeEvent<HTMLTextAreaElement>) => {
        setContent(e.target.value);
    };

    const handleCancel = () => {
        if (onCancel) {
            onCancel();
        }
        setContent('');
    };

    const remainingChars = MAX_COMMENT_LENGTH - content.length;
    const isExceeded = remainingChars < 0;

    return (
        <FormContainer onSubmit={handleSubmit}>
            <TextareaWrapper>
                <CommentTextarea
                    value={content}
                    onChange={handleChange}
                    placeholder={placeholder}
                    aria-label="评论内容"
                />
                <CharacterCount isExceeded={isExceeded}>
                    {remainingChars}
                </CharacterCount>
            </TextareaWrapper>

            <ButtonContainer>
                {showCancel && (
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
                    disabled={!content.trim() || isExceeded}
                >
                    {submitLabel}
                </Button>
            </ButtonContainer>
        </FormContainer>
    );
};

export default CommentForm; 