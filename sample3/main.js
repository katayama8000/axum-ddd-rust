var app = new Vue({
    el: '#app',
    data: {
        newTodo: "",
        todos: []
    },
    methods: {
        addTodo: function (event) {
            if (this.newTodo == '') return;
            const todo = {
                todo: this.newTodo,
                isDone: false
            };
            this.todos.push(todo);
            this.newTodo = ''
        },
        deleteTodo: function (index) {
            this.todos.splice(index, 1)
        }
    }
})