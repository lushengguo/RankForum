import React from 'react';
import styled, { css } from 'styled-components';
import { Theme } from '../../utils/theme';

// 按钮类型定义
export type ButtonVariant = 'primary' | 'secondary' | 'text' | 'outlined';
export type ButtonSize = 'small' | 'medium' | 'large';

interface ButtonProps {
    variant?: ButtonVariant;
    size?: ButtonSize;
    fullWidth?: boolean;
    disabled?: boolean;
    onClick?: React.MouseEventHandler<HTMLButtonElement>;
    type?: 'button' | 'submit' | 'reset';
    children: React.ReactNode;
    icon?: React.ReactNode;
    className?: string;
}

// 基础按钮样式
const StyledButton = styled.button<{
    variant: ButtonVariant;
    size: ButtonSize;
    fullWidth: boolean;
    theme: Theme;
    hasIcon: boolean;
}>`
  display: inline-flex;
  align-items: center;
  justify-content: center;
  position: relative;
  font-weight: ${props => props.theme.typography.fontWeightMedium};
  border-radius: ${props => props.theme.borderRadius.medium};
  transition: all ${props => props.theme.transitions.normal};
  width: ${props => (props.fullWidth ? '100%' : 'auto')};
  gap: ${props => props.theme.spacing.xsmall};
  
  &:disabled {
    cursor: not-allowed;
    opacity: 0.6;
  }
  
  /* 尺寸变体 */
  ${props => {
        switch (props.size) {
            case 'small':
                return css`
          font-size: ${props.theme.fontSizes.small};
          padding: ${props.theme.spacing.xxsmall} ${props.theme.spacing.small};
          height: 32px;
        `;
            case 'large':
                return css`
          font-size: ${props.theme.fontSizes.large};
          padding: ${props.theme.spacing.small} ${props.theme.spacing.large};
          height: 48px;
        `;
            default: // medium
                return css`
          font-size: ${props.theme.fontSizes.medium};
          padding: ${props.theme.spacing.xsmall} ${props.theme.spacing.medium};
          height: 36px;
        `;
        }
    }}
  
  /* 类型变体 */
  ${props => {
        switch (props.variant) {
            case 'primary':
                return css`
          background-color: ${props.theme.colors.primary};
          color: white;
          
          &:hover:not(:disabled) {
            background-color: rgba(26, 115, 232, 0.9);
            box-shadow: ${props.theme.shadows.small};
          }
          
          &:active:not(:disabled) {
            background-color: rgba(26, 115, 232, 0.8);
          }
        `;
            case 'secondary':
                return css`
          background-color: ${props.theme.colors.secondary};
          color: white;
          
          &:hover:not(:disabled) {
            background-color: rgba(52, 168, 83, 0.9);
            box-shadow: ${props.theme.shadows.small};
          }
          
          &:active:not(:disabled) {
            background-color: rgba(52, 168, 83, 0.8);
          }
        `;
            case 'outlined':
                return css`
          background-color: transparent;
          color: ${props.theme.colors.primary};
          border: 1px solid ${props.theme.colors.primary};
          
          &:hover:not(:disabled) {
            background-color: rgba(26, 115, 232, 0.04);
          }
          
          &:active:not(:disabled) {
            background-color: rgba(26, 115, 232, 0.12);
          }
        `;
            case 'text':
                return css`
          background-color: transparent;
          color: ${props.theme.colors.primary};
          
          &:hover:not(:disabled) {
            background-color: rgba(26, 115, 232, 0.04);
          }
          
          &:active:not(:disabled) {
            background-color: rgba(26, 115, 232, 0.12);
          }
        `;
            default:
                return '';
        }
    }}
`;

const IconWrapper = styled.span`
  display: inline-flex;
  align-items: center;
  justify-content: center;
`;

// 按钮组件
const Button: React.FC<ButtonProps> = ({
    variant = 'primary',
    size = 'medium',
    fullWidth = false,
    disabled = false,
    onClick,
    type = 'button',
    children,
    icon,
    className,
    ...rest
}) => {
    return (
        <StyledButton
            variant={variant}
            size={size}
            fullWidth={fullWidth}
            disabled={disabled}
            onClick={onClick}
            type={type}
            className={className}
            hasIcon={!!icon}
            {...rest}
        >
            {icon && <IconWrapper>{icon}</IconWrapper>}
            {children}
        </StyledButton>
    );
};

export default Button; 