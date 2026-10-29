import vine from '@vinejs/vine'

const taskValidator = vine.create(
    vine.object({
        title: vine.string(),
        description: vine.string(),
        status: vine.enum(['pending' , 'completed']),
    })    
)

export default taskValidator