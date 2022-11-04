var app = new Vue({
    el: '#app',
    data: {
        newTodo: "",
        todos: []
    },
    methods: {
        addTodo(event) {
            if (this.newTodo == '') return;
            const todo = {
                todo: this.newTodo,
                isDone: false
            };
            this.todos.push(todo);
            this.newTodo = ''
        },
        deleteTodo(index) {
            this.todos.splice(index, 1)
        }
    }
})