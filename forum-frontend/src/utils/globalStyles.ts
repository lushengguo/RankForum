import { createGlobalStyle } from 'styled-components';
import { Theme } from './theme';

const GlobalStyles = createGlobalStyle<{ theme: Theme }>`
  /* 重置浏览器样式 */
  * {
    margin: 0;
    padding: 0;
    box-sizing: border-box;
  }
  
  body {
    font-family: ${props => props.theme.typography.fontFamily};
    font-size: ${props => props.theme.fontSizes.medium};
    line-height: ${props => props.theme.typography.lineHeight};
    color: ${props => props.theme.colors.text};
    background-color: ${props => props.theme.colors.background};
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
  }
  
  a {
    color: ${props => props.theme.colors.link};
    text-decoration: none;
    transition: color ${props => props.theme.transitions.fast};
    
    &:hover {
      text-decoration: underline;
    }
  }
  
  button {
    font-family: ${props => props.theme.typography.fontFamily};
    border: none;
    cursor: pointer;
    background: none;
    
    &:focus {
      outline: none;
      box-shadow: ${props => props.theme.shadows.focus};
    }
  }
  
  input, textarea {
    font-family: ${props => props.theme.typography.fontFamily};
    font-size: ${props => props.theme.fontSizes.medium};
    
    &:focus {
      outline: none;
      box-shadow: ${props => props.theme.shadows.focus};
    }
  }
  
  ul, ol {
    list-style: none;
  }
  
  code {
    font-family: ${props => props.theme.typography.fontFamilyMonospace};
  }
  
  h1, h2, h3, h4, h5, h6 {
    font-weight: ${props => props.theme.typography.fontWeightMedium};
    line-height: 1.2;
    margin-bottom: ${props => props.theme.spacing.medium};
  }
  
  h1 {
    font-size: ${props => props.theme.fontSizes.xxxlarge};
  }
  
  h2 {
    font-size: ${props => props.theme.fontSizes.xxlarge};
  }
  
  h3 {
    font-size: ${props => props.theme.fontSizes.xlarge};
  }
  
  h4 {
    font-size: ${props => props.theme.fontSizes.large};
  }
  
  h5 {
    font-size: ${props => props.theme.fontSizes.medium};
  }
  
  h6 {
    font-size: ${props => props.theme.fontSizes.small};
  }
  
  p {
    margin-bottom: ${props => props.theme.spacing.medium};
  }
`;

export default GlobalStyles; 