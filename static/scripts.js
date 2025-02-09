document.addEventListener('DOMContentLoaded', () => {
    const topics = [
        'Rust 编程语言',
        'Web 开发',
        '人工智能',
        '机器学习',
        '数据科学'
    ];

    const topicsList = document.getElementById('topics-list');

    topics.forEach(topic => {
        const li = document.createElement('li');
        li.textContent = topic;
        topicsList.appendChild(li);
    });
});
