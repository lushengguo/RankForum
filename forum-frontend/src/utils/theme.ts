// Google/Apple 风格的设计系统
// 使用干净的色彩、适当的留白和简约的设计

export const theme = {
    colors: {
        // 主题色
        primary: '#1a73e8', // Google蓝
        secondary: '#34a853', // Google绿
        tertiary: '#fbbc05', // Google黄
        quaternary: '#ea4335', // Google红

        // 灰度系列
        darkGrey: '#202124',
        grey: '#5f6368',
        lightGrey: '#dadce0',
        veryLightGrey: '#f1f3f4',

        // 其他功能色
        background: '#ffffff',
        text: '#202124',
        link: '#1a73e8',
        error: '#d93025',
        success: '#34a853',
        warning: '#fbbc05',

        // 特殊用途色
        cardBackground: '#ffffff',
        headerBackground: '#ffffff',
        footerBackground: '#f1f3f4',
        divider: '#dadce0',
        shadow: 'rgba(60, 64, 67, 0.3)'
    },

    fontSizes: {
        xsmall: '0.75rem',    // 12px
        small: '0.875rem',    // 14px
        medium: '1rem',       // 16px
        large: '1.125rem',    // 18px
        xlarge: '1.25rem',    // 20px
        xxlarge: '1.5rem',    // 24px
        xxxlarge: '2rem',     // 32px
        huge: '2.5rem'        // 40px
    },

    spacing: {
        xxsmall: '0.25rem',   // 4px
        xsmall: '0.5rem',     // 8px
        small: '0.75rem',     // 12px
        medium: '1rem',       // 16px
        large: '1.5rem',      // 24px
        xlarge: '2rem',       // 32px
        xxlarge: '3rem',      // 48px
        xxxlarge: '4rem'      // 64px
    },

    borderRadius: {
        small: '0.25rem',     // 4px
        medium: '0.5rem',     // 8px
        large: '1rem',        // 16px
        pill: '9999px'
    },

    shadows: {
        small: '0 1px 2px 0 rgba(60, 64, 67, 0.3), 0 1px 3px 1px rgba(60, 64, 67, 0.15)',
        medium: '0 1px 3px 0 rgba(60, 64, 67, 0.3), 0 4px 8px 3px rgba(60, 64, 67, 0.15)',
        large: '0 2px 6px 2px rgba(60, 64, 67, 0.15), 0 1px 2px 0 rgba(60, 64, 67, 0.3)',
        focus: '0 0 0 2px rgba(26, 115, 232, 0.4)'
    },

    transitions: {
        fast: '0.1s ease-in-out',
        normal: '0.2s ease-in-out',
        slow: '0.3s ease-in-out'
    },

    typography: {
        fontFamily: '"Google Sans", "Roboto", "Helvetica", "Arial", sans-serif',
        fontFamilyMonospace: '"Roboto Mono", monospace',
        fontWeightLight: 300,
        fontWeightRegular: 400,
        fontWeightMedium: 500,
        fontWeightBold: 700,
        lineHeight: 1.5,
        letterSpacing: '0.00938em'
    },

    breakpoints: {
        xs: '0px',
        sm: '600px',
        md: '960px',
        lg: '1280px',
        xl: '1920px'
    }
};

// 类型定义
export type Theme = typeof theme;
export default theme; 