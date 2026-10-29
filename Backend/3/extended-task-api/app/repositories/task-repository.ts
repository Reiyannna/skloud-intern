import Task from '#models/task'

type CreateTaskData = {
    userId: number
    title: string
    description: string
    status: 'pending' | 'completed'
}

export default class TaskRepository{
    //function kay naay async

    async create(data: CreateTaskData){
        return await Task.create(data)
    } 

    async showAll(userId: number){
        return await Task.query().where('userId' , userId).orderBy('id', 'asc')
    }
    
}