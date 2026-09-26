import vine from '@vinejs/vine'

const statusRule = vine.enum([
  'pending',
  'in_progress',
  'completed',
] as const)

const nonEmptyString = vine
  .string()
  .trim()
  .minLength(1)

export const createTaskValidator = vine.compile(
  vine.object({
    title: nonEmptyString,
    description: vine.string().trim().optional(),
    status: statusRule.optional(),
  })
)

export const updateTaskValidator = vine.compile(
  vine.object({
    title: nonEmptyString.optional(),
    description: vine.string().trim().optional(),
    status: statusRule.optional(),
  })
)
