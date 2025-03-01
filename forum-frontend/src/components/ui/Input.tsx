import React, { forwardRef } from 'react';
import styled, { css } from 'styled-components';
import { Theme } from '../../utils/theme';

export type InputSize = 'small' | 'medium' | 'large';

interface InputProps extends Omit<React.InputHTMLAttributes<HTMLInputElement>, 'size'> {
    size?: InputSize;
    label?: string;
    helperText?: string;
    error?: boolean;
    fullWidth?: boolean;
    startIcon?: React.ReactNode;
    endIcon?: React.ReactNode;
}

const InputContainer = styled.div<{ fullWidth: boolean }>`
  display: flex;
  flex-direction: column;
  width: ${props => (props.fullWidth ? '100%' : 'auto')};
  margin-bottom: ${props => props.theme.spacing.medium};
`;

const StyledLabel = styled.label`
  margin-bottom: ${props => props.theme.spacing.xsmall};
  font-size: ${props => props.theme.fontSizes.small};
  color: ${props => props.theme.colors.grey};
  font-weight: ${props => props.theme.typography.fontWeightMedium};
`;

const InputWrapper = styled.div`
  position: relative;
  display: flex;
  align-items: center;
  width: 100%;
`;

const IconWrapper = styled.div<{ position: 'start' | 'end' }>`
  position: absolute;
  display: flex;
  align-items: center;
  justify-content: center;
  color: ${props => props.theme.colors.grey};
  ${props => props.position === 'start' && 'left: 12px;'}
  ${props => props.position === 'end' && 'right: 12px;'}
  pointer-events: none;
`;

const StyledInput = styled.input<{
    size: InputSize;
    error: boolean;
    hasStartIcon: boolean;
    hasEndIcon: boolean;
    theme: Theme;
}>`
  width: 100%;
  border: 1px solid ${props => props.error ? props.theme.colors.error : props.theme.colors.lightGrey};
  border-radius: ${props => props.theme.borderRadius.medium};
  font-family: ${props => props.theme.typography.fontFamily};
  transition: all ${props => props.theme.transitions.fast};
  
  &:focus {
    border-color: ${props => props.error ? props.theme.colors.error : props.theme.colors.primary};
    box-shadow: ${props => props.error
        ? `0 0 0 1px ${props.theme.colors.error}`
        : props.theme.shadows.focus};
  }
  
  &:hover:not(:focus):not(:disabled) {
    border-color: ${props => props.error ? props.theme.colors.error : props.theme.colors.grey};
  }
  
  &:disabled {
    background-color: ${props => props.theme.colors.veryLightGrey};
    cursor: not-allowed;
    opacity: 0.7;
  }
  
  &::placeholder {
    color: ${props => props.theme.colors.grey};
    opacity: 0.7;
  }
  
  /* 根据是否有图标调整padding */
  padding-left: ${props => props.hasStartIcon ? '2.5rem' : '0.75rem'};
  padding-right: ${props => props.hasEndIcon ? '2.5rem' : '0.75rem'};
  
  /* 尺寸变体 */
  ${props => {
        switch (props.size) {
            case 'small':
                return css`
          font-size: ${props.theme.fontSizes.small};
          height: 32px;
          padding-top: ${props.theme.spacing.xxsmall};
          padding-bottom: ${props.theme.spacing.xxsmall};
        `;
            case 'large':
                return css`
          font-size: ${props.theme.fontSizes.medium};
          height: 48px;
          padding-top: ${props.theme.spacing.small};
          padding-bottom: ${props.theme.spacing.small};
        `;
            default: // medium
                return css`
          font-size: ${props.theme.fontSizes.medium};
          height: 40px;
          padding-top: ${props.theme.spacing.xsmall};
          padding-bottom: ${props.theme.spacing.xsmall};
        `;
        }
    }}
`;

const HelperText = styled.div<{ error: boolean }>`
  font-size: ${props => props.theme.fontSizes.xsmall};
  color: ${props => props.error ? props.theme.colors.error : props.theme.colors.grey};
  margin-top: ${props => props.theme.spacing.xxsmall};
`;

const Input = forwardRef<HTMLInputElement, InputProps>((props, ref) => {
    const {
        size = 'medium',
        label,
        helperText,
        error = false,
        fullWidth = false,
        startIcon,
        endIcon,
        ...rest
    } = props;

    return (
        <InputContainer fullWidth={fullWidth}>
            {label && <StyledLabel>{label}</StyledLabel>}
            <InputWrapper>
                {startIcon && <IconWrapper position="start">{startIcon}</IconWrapper>}
                <StyledInput
                    ref={ref}
                    size={size}
                    error={error}
                    hasStartIcon={!!startIcon}
                    hasEndIcon={!!endIcon}
                    {...rest}
                />
                {endIcon && <IconWrapper position="end">{endIcon}</IconWrapper>}
            </InputWrapper>
            {helperText && <HelperText error={error}>{helperText}</HelperText>}
        </InputContainer>
    );
});

Input.displayName = 'Input';

export default Input; 