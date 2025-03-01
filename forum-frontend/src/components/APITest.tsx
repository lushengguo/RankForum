import React, { useState, useEffect } from 'react';
import { fieldAPI, queryAPI } from '../services/api';

const APITest: React.FC = () => {
  const [fieldData, setFieldData] = useState<any>(null);
  const [fieldAddressData, setFieldAddressData] = useState<string | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    async function fetchData() {
      setLoading(true);
      setError(null);
      
      try {
        // 尝试获取所有字段
        const fields = await fieldAPI.getAllFields();
        setFieldData(fields);
        
        // 尝试获取一个特定字段地址
        if (fields && fields.length > 0) {
          const fieldName = fields[0].name;
          const address = await queryAPI.getFieldAddress(fieldName);
          setFieldAddressData(address);
        } else {
          // 如果没有字段，尝试创建一个
          const result = await fieldAPI.createField("测试领域");
          console.log("创建字段结果:", result);
          
          // 再次尝试获取字段列表
          const updatedFields = await fieldAPI.getAllFields();
          setFieldData(updatedFields);
        }
      } catch (err) {
        console.error("API测试失败:", err);
        setError(`API请求失败: ${err instanceof Error ? err.message : String(err)}`);
      } finally {
        setLoading(false);
      }
    }
    
    fetchData();
  }, []);

  return (
    <div style={{ padding: '20px', maxWidth: '800px', margin: '0 auto' }}>
      <h2>后端API连接测试</h2>
      
      {loading && <p>正在加载数据...</p>}
      
      {error && (
        <div style={{ padding: '10px', backgroundColor: '#ffdddd', border: '1px solid #ff0000', borderRadius: '4px' }}>
          <h3>错误信息</h3>
          <p>{error}</p>
        </div>
      )}
      
      {!loading && !error && (
        <div>
          <h3>后端连接成功!</h3>
          
          <div style={{ marginTop: '20px' }}>
            <h4>字段数据</h4>
            {fieldData && fieldData.length > 0 ? (
              <ul>
                {fieldData.map((field: any, index: number) => (
                  <li key={index}>
                    名称: {field.name}, 地址: {field.address}
                  </li>
                ))}
              </ul>
            ) : (
              <p>未找到任何字段数据</p>
            )}
          </div>
          
          {fieldAddressData && (
            <div style={{ marginTop: '20px' }}>
              <h4>字段地址查询结果</h4>
              <p>{fieldAddressData}</p>
            </div>
          )}
        </div>
      )}
    </div>
  );
};

export default APITest; 