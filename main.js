new Vue({
    el: '#app',
    data: {
        items: [
            { name: 'りんご', color: '赤色' },
            { name: 'バナナ', color: '黄色' },
            { name: 'すいか', color: '赤色' }
        ]
    }
});

new Vue({
    el: '#app2',
    data: {
        items: [
            // 名前、年齢
            { name: 'mike', age: 10 },
            { name: 'nancy', age: 20 },
            { name: 'tom', age: 30 }
        ]
    }
});