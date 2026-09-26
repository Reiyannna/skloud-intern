import type { HttpContext } from '@adonisjs/core/http'
import Task from '#models/task'
import {
  createTaskValidator,
  updateTaskValidator,
} from '#validators/task'

export default class TasksController {
  /**
   * GET /tasks
   * Return all tasks
   */
  async index({ response }: HttpContext) {
    const tasks = await Task.all()

    return response.ok(tasks)
  }

  /**
   * GET /tasks/:id
   * Return one task
   */
  async show({ params, response }: HttpContext) {
    const task = await Task.find(params.id)

    if (!task) {
      return response.notFound({
        message: 'Task not found',
      })
    }

    return response.ok(task)
  }

  /**
   * POST /tasks
   * Validate and create a task
   */
  async store({ request, response }: HttpContext) {
    try {
      const payload = await request.validateUsing(createTaskValidator)

      const task = await Task.create({
        title: payload.title,
        description: payload.description ?? null,
        status: payload.status ?? 'pending',
      })

      return response.created(task)
    } catch (error: any) {
      return response.badRequest({
        message: 'Invalid request',
        errors: error.messages ?? [],
      })
    }
  }

  /**
   * PATCH /tasks/:id
   * Update an existing task
   */
  async update({ params, request, response }: HttpContext) {
    const task = await Task.find(params.id)

    if (!task) {
      return response.notFound({
        message: 'Task not found',
      })
    }

    try {
      const payload = await request.validateUsing(updateTaskValidator)

        console.log('PAYLOAD:', payload)
        console.log('TITLE:', JSON.stringify(payload.title))
        console.log('LENGTH:', payload.title?.length)

      if (payload.title !== undefined && payload.title.trim() === '') {
        return response.badRequest({
          message: 'Invalid request',
          errors: [
            {
              message: 'The title field must not be empty',
              field: 'title',
            },
          ],
        })
      }

      task.merge(payload)

      await task.save()

      return response.ok(task)
    } catch (error: any) {
      return response.badRequest({
        message: 'Invalid request',
        errors: error.messages ?? [],
      })
    }
  }

  /**
   * DELETE /tasks/:id
   * Delete an existing task
   */
  async destroy({ params, response }: HttpContext) {
    const task = await Task.find(params.id)

    if (!task) {
      return response.notFound({
        message: 'Task not found',
      })
    }

    await task.delete()

    return response.noContent()
  }
}
