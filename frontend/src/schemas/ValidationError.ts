import { z } from 'zod';

export const validationErrorSchema = z.object({
  fields: z.array(
    z.object({
      name: z.string(),
      keys: z.array(
        z.enum([
          'validation.required',
          'validation.duplicate',
          'validation.length_equals',
          'validation.numeric_only',
          'validation.ulid',
          'validation.resource_not_found',
          'validation.max_length',
        ]),
      ),
    }),
  ),
});
export type ValidationError = z.infer<typeof validationErrorSchema>;

export const isValidationError = (
  source: unknown,
): source is ValidationError => {
  return validationErrorSchema.safeParse(source).success;
};
