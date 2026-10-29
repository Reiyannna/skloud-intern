import type { HttpContext } from '@adonisjs/core/http'
import taskValidator from '#validators/task'    
import TaskRepository from '../repositories/task-repository.ts'

export default class TasksController {
    taskRepository = new TaskRepository()

    async store({response, request, auth} : HttpContext ){
        const user = auth.getUserOrFail()

        const data = await request.validateUsing(taskValidator)
        const task = await this.taskRepository.create({...data , userId: user.id})  
        return response.status(201).json(task)
    }

    async showAll({auth, response} : HttpContext) {
        const user = auth.getUserOrFail()

        const data = await this.taskRepository.showAll(user.id)
        return response.status(200).json(data)
    }

}