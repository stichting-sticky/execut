import { glob } from 'astro/loaders'
import { defineCollection, reference, z, type SchemaContext } from 'astro:content'

export type Role = z.infer<typeof Role>

export const roles = [
  'chair',
  'treasurer',
  'acquisition',
  'speakers',
  'location',
  'promotion',
  'board',
] as const

const Role = z.enum(roles)

export type Committee = z.infer<typeof Committee>

const Committee = z.object({
  name: z.string(),
  role: Role,
  contact: z.coerce.string().url().optional(),
}).array()

export type Tier = z.infer<typeof Tier>

export const tiers = ['platinum', 'gold', 'silver', 'bronze', 'introduction'] as const

const Tier = z.enum(tiers)

export type Programme = z.infer<typeof Programme>

const Programme = z.object({
  time: z.string(),
  common: z.string().optional(),
  activities: z.discriminatedUnion('type', [
    z.object({
      type: z.literal('talk'),
      activity: reference('talks'),
    }),
    z.object({
      type: z.literal('workshop'),
      activity: reference('workshops'),
    }),
  ]).array().optional(),
}).refine(({ common, activities }) => !common !== !activities?.length).array()

export type Edition = z.infer<typeof Edition>

const Edition = z.object({
  date: z.coerce.date(),
  programme: Programme.optional(),
  speakers: reference('speakers')
    .array()
    .optional()
    .transform((val) => val ?? []),
  hosts: reference('hosts')
    .array()
    .optional(),
  venue: reference('venues').optional(),
  partners: z.record(Tier, reference('partners').array())
    .optional()
    .transform((val) => val ?? {}),
  committee: Committee,
})

export type Social = z.infer<typeof Social>

export const socials = [
  'bluesky',
  'facebook',
  'github',
  'glassdoor',
  'instagram',
  'linkedin',
  'twitter',
  'youtube',
  'x',
] as const

const Social = z.enum(socials)

export type Host = z.infer<ReturnType<typeof Host>>

const Host = ({ image }: SchemaContext) => z.object({
  name: z.string(),
  description: z.string().optional(),
  portrait: image(),
  socials: z.record(Social, z.coerce.string().url())
    .array()
    .optional()
    .transform((val) => val ?? {}),
})

export type Partner = z.infer<ReturnType<typeof Partner>>

const Partner = ({ image }: SchemaContext) => z.object({
  name: z.string(),
  industry: z.string(),
  logo: image(),
  contact: z.object({
    website: z.coerce.string().url().optional(),
    mail: z.coerce.string().email().optional(),
    socials: z.record(Social, z.coerce.string().url())
      .array()
      .optional()
      .transform((val) => val ?? []),
  }),
})

export type Speaker = z.infer<ReturnType<typeof Speaker>>

const Speaker = ({ image }: SchemaContext) => z.object({
  name: z.string(),
  description: z.string().optional(),
  portrait: image(),
  socials: z.record(Social, z.coerce.string().url())
    .array()
    .optional()
    .transform((val) => val ?? {}),
})

export type Talk = z.infer<typeof Talk>

const Talk = z.object({
  title: z.string(),
  description: z.string().optional(),
  tags: z.string()
    .array()
    .optional()
    .transform((val) => val ?? []),
  speaker: reference('speakers'),
})

export type Venue = z.infer<ReturnType<typeof Venue>>

const Venue = ({ image }: SchemaContext) => z.object({
  name: z.string(),
  location: z.string(),
  image: image(),
  address: z.string().transform((val) => val.split('\n')),
  directions: z.coerce.string().url(),
  embed: z.coerce.string().url(),
})

export type Workshop = z.infer<typeof Workshop>

const Workshop = z.object({
  title: z.string(),
  description: z.string().optional(),
  tags: z.string()
    .array()
    .optional()
    .transform((val) => val ?? []),
  organizer: reference('partners'),
})

const editions = defineCollection({
  loader: glob({ pattern: '**/[^_]*.{yaml,yml}', base: './src/content/editions' }),
  schema: Edition,
})

const partners = defineCollection({
  loader: glob({ pattern: '**/[^_]*.md', base: './src/content/partners' }),
  schema: Partner,
})

const hosts = defineCollection({
  loader: glob({ pattern: '**/[^_]*.md', base: './src/content/hosts' }),
  schema: Speaker,
})

const speakers = defineCollection({
  loader: glob({ pattern: '**/[^_]*.md', base: './src/content/speakers' }),
  schema: Speaker,
})

const talks = defineCollection({
  loader: glob({ pattern: '**/[^_]*.md', base: './src/content/talks' }),
  schema: Talk,
})

const venues = defineCollection({
  loader: glob({ pattern: '**/[^_]*.md', base: './src/content/venues' }),
  schema: Venue,
})

const workshops = defineCollection({
  loader: glob({ pattern: '**/[^_]*.md', base: './src/content/workshops' }),
  schema: Workshop,
})

export const collections = {
  editions,
  partners,
  hosts,
  speakers,
  talks,
  venues,
  workshops,
}
