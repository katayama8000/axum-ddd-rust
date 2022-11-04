let vm1 = new Vue({
    el: '#app',
    data: {
        items: [
            { name: 'りんご', color: '赤色' },
            { name: 'バナナ', color: '黄色' },
            { name: 'すいか', color: '赤色' }
        ]
    }
});

console.log(vm1)

new Vue({
    el: '#app2',
    data: {
        items: [
            // 名前、年齢
            { name: 'mike', age: 10 },
            { name: 'nancy', age: 20 },
            { name: 'tom', age: 30 }
        ]
    },
    methods: {
        changeMessage1: function () {
            vm1.items[0].name = 'changed'
        }
    },
    template: `<h1>テンプレート,{{items[0].name}}</h1>`
});

new Vue({
    el: '#app3',
    data: {
        name: 'mike',
    },
    render(h) {
        return h('h1', 'render関数')
    }
});