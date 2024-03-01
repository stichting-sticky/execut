import { defineCollection, reference, z } from 'astro:content'

export const roles = [
  'chair',
  'treasurer',
  'acquisition',
  'speakers',
  'location',
  'promotion',
  'board',
] as const

export const tiers = [
  'platinum',
  'gold',
  'silver',
  'bronze',
] as const

const editions = defineCollection({
  type: 'data',
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
    hosts: reference('speakers').array().optional(),
    speakers: reference('speakers').array().optional(),
    talks: reference('talks').array().optional(),
    partners: z.record(z.enum(tiers), reference('partners').array()).optional(),
    workshops: reference('workshops').array().optional(),
    venue: reference('venues'),
    committee: z.object({
      name: z.string(),
      role: z.enum(roles),
      href: z.string().url().optional(),
    }).array(),
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
  type: 'content',
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
  type: 'content',
  schema: ({ image }) =>
    z.object({
      name: z.string(),
      description: z.string(),
      portrait: image(),
      // Transform `boolean | undefined` to `boolean`
      host: z
        .boolean()
        .optional()
        .transform((val) => !!val),
    }),
})

const talks = defineCollection({
  type: 'content',
  schema: z.object({
    speaker: reference('speakers'),
    title: z.string(),
  }),
})

const venues = defineCollection({
  type: 'content',
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
  type: 'content',
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
