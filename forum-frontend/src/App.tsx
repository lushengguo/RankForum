import React, { useEffect, useState } from 'react';
import { Routes, Route, Link } from 'react-router-dom';
import styled from 'styled-components';
import FieldList from './components/field/FieldList';
import PostForm from './components/post/PostForm';
import PostDetail from './components/post/PostDetail';
import { postAPI, queryAPI } from './services/api';
import { Field, Post, OrderingType } from './types';
import APITest from './components/APITest';

// 创建一个扩展的Field接口，包含可选的描述和帖子数量
interface ExtendedField extends Field {
  description?: string;
  postCount?: number;
}

// 创建样式组件
const AppContainer = styled.div`
  max-width: 1200px;
  margin: 0 auto;
  padding: 0 20px;
`;

const Header = styled.header`
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 20px 0;
  border-bottom: 1px solid ${props => props.theme.colors.lightGrey};
  margin-bottom: 30px;
`;

const Logo = styled.h1`
  font-size: ${props => props.theme.fontSizes.xxlarge};
  color: ${props => props.theme.colors.primary};
  margin: 0;
`;

const Navigation = styled.nav`
  display: flex;
  gap: 20px;
`;

const NavLink = styled(Link)`
  text-decoration: none;
  color: ${props => props.theme.colors.text};
  font-weight: 500;
  font-size: ${props => props.theme.fontSizes.medium};
  transition: color 0.2s;
  
  &:hover {
    color: ${props => props.theme.colors.primary};
  }
`;

const MainContent = styled.main`
  min-height: 70vh;
`;

const LoadingIndicator = styled.div`
  text-align: center;
  padding: 2rem;
  color: ${props => props.theme.colors.grey};
`;

const ErrorMessage = styled.div`
  background-color: ${props => props.theme.colors.error};
  color: white;
  padding: 1rem;
  border-radius: 4px;
  margin-bottom: 1rem;
`;

const PostCard = styled(Link)`
  display: block;
  text-decoration: none;
  color: inherit;
  background-color: white;
  border-radius: 8px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  margin-bottom: 16px;
  padding: 16px;
  transition: transform 0.2s, box-shadow 0.2s;
  
  &:hover {
    transform: translateY(-4px);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  }
`;

const PostTitle = styled.h3`
  margin-top: 0;
  color: ${props => props.theme.colors.primary};
`;

const PostMeta = styled.div`
  display: flex;
  justify-content: space-between;
  color: ${props => props.theme.colors.grey};
  font-size: 0.9rem;
  margin-top: 12px;
`;

// 首页内容组件
const Home = () => {
  const [fields, setFields] = useState<ExtendedField[]>([]);
  const [hotPosts, setHotPosts] = useState<Post[]>([]);
  const [isLoadingFields, setIsLoadingFields] = useState(true);
  const [isLoadingPosts, setIsLoadingPosts] = useState(true);
  const [error, setError] = useState<string | null>(null);
  
  // 获取领域数据
  useEffect(() => {
    const fetchFieldsData = async () => {
      setIsLoadingFields(true);
      try {
        // 这里假设有一个获取所有领域的API
        // 由于当前API似乎没有直接提供获取所有字段的方法，我们模拟几个字段
        const fieldNames = ["技术讨论", "生活分享", "学术研究"];
        const fieldsData = await Promise.all(
          fieldNames.map(async (name) => {
            try {
              const address = await queryAPI.getFieldAddress(name);
              return { 
                name, 
                address, 
                description: `${name}领域` 
              } as ExtendedField;
            } catch (err) {
              console.error(`获取字段 ${name} 地址失败:`, err);
              return null;
            }
          })
        );
        
        const validFields = fieldsData.filter((field): field is ExtendedField => field !== null);
        setFields(validFields);
      } catch (err) {
        console.error("获取字段数据失败:", err);
        setError("获取领域数据失败，请刷新页面重试");
      } finally {
        setIsLoadingFields(false);
      }
    };
    
    fetchFieldsData();
  }, []);
  
  // 获取热门帖子
  useEffect(() => {
    const fetchHotPosts = async () => {
      setIsLoadingPosts(true);
      try {
        // 获取热门帖子，按分数排序
        const posts = await postAPI.filterPosts(undefined, undefined, {
          ordering: OrderingType.ByScore as any,
          ascending: false,
          max_results: 5
        });
        
        setHotPosts(posts);
      } catch (err) {
        console.error("获取热门帖子失败:", err);
        setError("获取热门帖子数据失败，请刷新页面重试");
      } finally {
        setIsLoadingPosts(false);
      }
    };
    
    fetchHotPosts();
  }, []);
  
  // 格式化时间戳为易读格式
  const formatDate = (timestamp: number): string => {
    const date = new Date(timestamp * 1000);
    return date.toLocaleDateString('zh-CN');
  };
  
  return (
    <div>
      <h2>欢迎来到RankForum - 高质量在线论坛</h2>
      <p>在这里，您可以浏览各个领域的讨论，发表自己的观点，与其他用户互动。</p>
      
      {error && <ErrorMessage>{error}</ErrorMessage>}
      
      <h3>热门领域</h3>
      {isLoadingFields ? (
        <LoadingIndicator>加载领域数据中...</LoadingIndicator>
      ) : (
        <FieldList fields={fields} />
      )}
      
      <Link to="/fields">查看全部领域 &rarr;</Link>
      
      <h3>热门帖子</h3>
      {isLoadingPosts ? (
        <LoadingIndicator>加载热门帖子中...</LoadingIndicator>
      ) : (
        <div>
          {hotPosts.length > 0 ? (
            hotPosts.map(post => (
              <PostCard key={post.address} to={`/post/${post.address}`}>
                <PostTitle>{post.title}</PostTitle>
                <p>{post.content.length > 100 ? `${post.content.substring(0, 100)}...` : post.content}</p>
                <PostMeta>
                  <span>点赞: {post.upvote}</span>
                  <span>评论: {post.comments.length}</span>
                  <span>发布于: {formatDate(post.timestamp)}</span>
                </PostMeta>
              </PostCard>
            ))
          ) : (
            <p>暂无热门帖子</p>
          )}
        </div>
      )}
    </div>
  );
};

// 字段列表页面
const FieldsPage = () => {
  const [fields, setFields] = useState<ExtendedField[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  
  useEffect(() => {
    const fetchAllFields = async () => {
      setIsLoading(true);
      try {
        // 这里假设有一个获取所有领域的API
        // 由于当前API似乎没有直接提供获取所有字段的方法，我们模拟一些字段
        const fieldNames = ["技术讨论", "生活分享", "学术研究", "艺术创作", "健康生活"];
        const fieldsData = await Promise.all(
          fieldNames.map(async (name) => {
            try {
              const address = await queryAPI.getFieldAddress(name);
              // 获取该领域的帖子数量
              let postCount = 0;
              try {
                const posts = await postAPI.filterPosts(name, address);
                postCount = posts.length;
              } catch (e) {
                console.error(`获取 ${name} 帖子数量失败:`, e);
              }
              
              return { 
                name, 
                address,
                description: `${name}领域的讨论区`,
                postCount
              } as ExtendedField;
            } catch (err) {
              console.error(`获取字段 ${name} 地址失败:`, err);
              return null;
            }
          })
        );
        
        const validFields = fieldsData.filter((field): field is ExtendedField => field !== null);
        setFields(validFields);
      } catch (err) {
        console.error("获取所有字段数据失败:", err);
        setError("获取领域数据失败，请刷新页面重试");
      } finally {
        setIsLoading(false);
      }
    };
    
    fetchAllFields();
  }, []);
  
  return (
    <div>
      <h2>所有领域</h2>
      <p>浏览所有可用的领域，点击进入感兴趣的领域参与讨论。</p>
      
      {error && <ErrorMessage>{error}</ErrorMessage>}
      
      {isLoading ? (
        <LoadingIndicator>加载领域数据中...</LoadingIndicator>
      ) : (
        <FieldList fields={fields} />
      )}
    </div>
  );
};

// 发布新帖页面
const NewPostPage = () => {
  const [fields, setFields] = useState<ExtendedField[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  
  useEffect(() => {
    const fetchAvailableFields = async () => {
      setIsLoading(true);
      try {
        // 获取可用的领域列表
        const fieldNames = ["技术讨论", "生活分享", "学术研究"];
        const fieldsData = await Promise.all(
          fieldNames.map(async (name) => {
            try {
              const address = await queryAPI.getFieldAddress(name);
              return { name, address } as ExtendedField;
            } catch (err) {
              console.error(`获取字段 ${name} 地址失败:`, err);
              return null;
            }
          })
        );
        
        const validFields = fieldsData.filter((field): field is ExtendedField => field !== null);
        setFields(validFields);
      } catch (err) {
        console.error("获取可用领域失败:", err);
        setError("获取可用领域失败，请刷新页面重试");
      } finally {
        setIsLoading(false);
      }
    };
    
    fetchAvailableFields();
  }, []);
  
  const handleSubmit = async (title: string, content: string, fieldAddress: string) => {
    try {
      // 找到对应的领域名称
      const field = fields.find(f => f.address === fieldAddress);
      if (!field) {
        throw new Error("无效的领域地址");
      }
      
      // 调用API发布帖子
      await postAPI.createPost(field.name, field.address, title, content);
      alert("发帖成功！");
      // 这里可以添加导航到首页或者该领域的帖子列表页面
    } catch (err) {
      console.error("发帖失败:", err);
      alert(`发帖失败: ${err instanceof Error ? err.message : "未知错误"}`);
    }
  };
  
  return (
    <div>
      <h2>发布新帖</h2>
      
      {error && <ErrorMessage>{error}</ErrorMessage>}
      
      {isLoading ? (
        <LoadingIndicator>加载可用领域中...</LoadingIndicator>
      ) : (
        <PostForm 
          onSubmit={handleSubmit} 
          availableFields={fields}
        />
      )}
    </div>
  );
};

const App: React.FC = () => {
  return (
    <AppContainer>
      <Header>
        <Logo>RankForum</Logo>
        <Navigation>
          <NavLink to="/">首页</NavLink>
          <NavLink to="/fields">领域</NavLink>
          <NavLink to="/new-post">发帖</NavLink>
          <NavLink to="/login">登录</NavLink>
          <NavLink to="/api-test">接口测试</NavLink>
        </Navigation>
      </Header>
      
      <MainContent>
        <Routes>
          <Route path="/" element={<Home />} />
          <Route path="/fields" element={<FieldsPage />} />
          <Route path="/new-post" element={<NewPostPage />} />
          <Route path="/post/:postId" element={<PostDetail />} />
          <Route path="/api-test" element={<APITest />} />
        </Routes>
      </MainContent>
    </AppContainer>
  );
};

export default App; 