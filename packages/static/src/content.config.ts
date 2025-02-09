import { glob } from 'astro/loaders'
import { z } from 'astro/zod'
import { defineCollection, reference } from 'astro:content'

export const roles = [
  'chair',
  'treasurer',
  'acquisition',
  'speakers',
  'location',
  'promotion',
  'board',
] as const

export const tiers = ['platinum', 'gold', 'silver', 'bronze'] as const

const editions = defineCollection({
  loader: glob({ pattern: '**/*.yml', base: './src/content/editions' }),
  schema: z.object({
    name: z.string(),
    date: z.date(),
    programme: z.discriminatedUnion('type', [
      z.object({
        time: z.string(),
        type: z.literal('common'),
        title: z.string(),
      }),
      z.object({
        time: z.string(),
        type: z.literal('talk'),
        activities: z.discriminatedUnion('type', [
          z.object({
            type: z.literal('talk'),
            activity: reference('talks'),
          }),
          z.object({
            type: z.literal('workshop'),
            activity: reference('workshops'),
          }),
        ]).array(),
      }),
    ]).array().optional(),
    hosts: reference('speakers')
      .array()
      .optional(),
    speakers: reference('speakers')
      .array()
      .optional(),
    talks: reference('talks')
      .array()
      .optional(),
    workshops: reference('workshops')
      .array()
      .optional(),
    partners: z.record(z.enum(tiers), reference('partners').array())
      .optional(),
    venue: reference('venues'),
    committee: z.object({
      name: z.string(),
      role: z.enum(roles),
      href: z.string().url().optional(),
    }).array().optional(),
  }),
})

export const socials = [
  'facebook',
  'github',
  'glassdoor',
  'instagram',
  'linkedin',
  'twitter',
  'youtube',
] as const

const partners = defineCollection({
  loader: glob({ pattern: '**/[^_]*.md', base: './src/content/partners' }),
  schema: ({ image }) =>
    z.object({
      name: z.string(),
      industry: z.string(),
      logo: image(),
      contact: z.object({
        website: z.string().url().optional(),
        mail: z.string().email().optional(),
        socials: z.record(z.enum(socials), z.string().url()).array(),
      }),
    }),
})

const speakers = defineCollection({
  loader: glob({ pattern: '**/[^_]*.md', base: './src/content/speakers' }),
  schema: ({ image }) => z.object({
    name: z.string(),
    description: z.string().optional(),
    portrait: image().optional(),
    // Transform `boolean | undefined` to `boolean` with the default value `false`
    host: z
      .boolean()
      .optional()
      .transform((val) => !!val),
  }),
})

const talks = defineCollection({
  loader: glob({ pattern: '**/[^_]*.md', base: './src/content/talks' }),
  schema: z.object({
    speaker: reference('speakers'),
    title: z.string(),
  }),
})

const venues = defineCollection({
  loader: glob({ pattern: '**/[^_]*.md', base: './src/content/venues' }),
  schema: ({ image }) => z.object({
    name: z.string(),
    location: z.string(),
    image: image(),
    address: z.string(),
    directions: z.string().url(),
    embed: z.string().url(),
  }),
})

const workshops = defineCollection({
  loader: glob({ pattern: '**/[^_]*.md', base: './src/content/workshops' }),
  schema: z.object({
    organizer: reference('partners'),
    title: z.string(),
  }),
})

export const collections = {
  editions,
  partners,
  speakers,
  talks,
  venues,
  workshops,
}
