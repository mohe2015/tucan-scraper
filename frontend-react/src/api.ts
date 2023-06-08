// This file is automatically generated at startup. Do not modify.
import { genericFetch } from "./api_base"
export async function course(input: string): Promise<WithTucanUrlW0NvbXBsZXRlQ291cnNlLCBDb3Vyc2VHcm91cFtdLCBDb3Vyc2VFdmVudFtdLCBNYXliZUNvbXBsZXRlTW9kdWxlW10sIHN0cmluZ10> {
        return await genericFetch("http://localhost:8080/course", input) as WithTucanUrlW0NvbXBsZXRlQ291cnNlLCBDb3Vyc2VHcm91cFtdLCBDb3Vyc2VFdmVudFtdLCBNYXliZUNvbXBsZXRlTW9kdWxlW10sIHN0cmluZ10
}
export async function course_group(input: string): Promise<WithTucanUrlW0NvbXBsZXRlQ291cnNlLCBDb3Vyc2VHcm91cCwgQ291cnNlR3JvdXBFdmVudFtdLCBzdHJpbmdd> {
        return await genericFetch("http://localhost:8080/course-group", input) as WithTucanUrlW0NvbXBsZXRlQ291cnNlLCBDb3Vyc2VHcm91cCwgQ291cnNlR3JvdXBFdmVudFtdLCBzdHJpbmdd
}
export async function courses(input: string | null): Promise<WithTucanUrlW1ZWTWVudUl0ZW0sIFZWTWVudUl0ZW1bXSwgTWF5YmVDb21wbGV0ZUNvdXJzZVtdLCBWVk1lbnVQYXRoUGFydFtdW11d> {
        return await genericFetch("http://localhost:8080/courses", input) as WithTucanUrlW1ZWTWVudUl0ZW0sIFZWTWVudUl0ZW1bXSwgTWF5YmVDb21wbGV0ZUNvdXJzZVtdLCBWVk1lbnVQYXRoUGFydFtdW11d
}
export async function exam(input: string): Promise<WithTucanUrlW0V4YW0sIE1heWJlQ29tcGxldGVNb2R1bGVbXSwgTWF5YmVDb21wbGV0ZUNvdXJzZVtdXQ> {
        return await genericFetch("http://localhost:8080/exam", input) as WithTucanUrlW0V4YW0sIE1heWJlQ29tcGxldGVNb2R1bGVbXSwgTWF5YmVDb21wbGV0ZUNvdXJzZVtdXQ
}
export async function get_modules(input: string | null): Promise<WithTucanUrlTW9kdWxlTWVudVJlc3BvbnNl> {
        return await genericFetch("http://localhost:8080/modules", input) as WithTucanUrlTW9kdWxlTWVudVJlc3BvbnNl
}
export async function index(input: null): Promise<string> {
        return await genericFetch("http://localhost:8080/", input) as string
}
export async function login(input: Login): Promise<LoginResult> {
        return await genericFetch("http://localhost:8080/login", input) as LoginResult
}
export async function logout(input: null): Promise<null> {
        return await genericFetch("http://localhost:8080/logout", input) as null
}
export async function module(input: string): Promise<WithTucanUrlTW9kdWxlUmVzcG9uc2U> {
        return await genericFetch("http://localhost:8080/module", input) as WithTucanUrlTW9kdWxlUmVzcG9uc2U
}
export async function my_courses(input: null): Promise<WithTucanUrlW01heWJlQ29tcGxldGVDb3Vyc2VbXSwgQ291cnNlR3JvdXBbXV0> {
        return await genericFetch("http://localhost:8080/my-courses", input) as WithTucanUrlW01heWJlQ29tcGxldGVDb3Vyc2VbXSwgQ291cnNlR3JvdXBbXV0
}
export async function my_exams(input: null): Promise<WithTucanUrlW1tNYXliZUNvbXBsZXRlTW9kdWxlLCBFeGFtXVtdLCBbTWF5YmVDb21wbGV0ZUNvdXJzZSwgRXhhbV1bXV0> {
        return await genericFetch("http://localhost:8080/my-exams", input) as WithTucanUrlW1tNYXliZUNvbXBsZXRlTW9kdWxlLCBFeGFtXVtdLCBbTWF5YmVDb21wbGV0ZUNvdXJzZSwgRXhhbV1bXV0
}
export async function my_modules(input: null): Promise<WithTucanUrlTWF5YmVDb21wbGV0ZU1vZHVsZVtd> {
        return await genericFetch("http://localhost:8080/my-modules", input) as WithTucanUrlTWF5YmVDb21wbGV0ZU1vZHVsZVtd
}
export async function search_course(input: string): Promise<SearchResult[]> {
        return await genericFetch("http://localhost:8080/search-course", input) as SearchResult[]
}
export async function search_module(input: string): Promise<SearchResult[]> {
        return await genericFetch("http://localhost:8080/search-modules", input) as SearchResult[]
}
export async function search_module_opensearch(input: string): Promise<SearchResult[]> {
        return await genericFetch("http://localhost:8080/search-modules-opensearch", input) as SearchResult[]
}
export type CompleteCourse =
{
  tucan_id: string,
  tucan_last_checked: string,
  title: string,
  course_id: string,
  sws: number,
  content: string,
}
export type CompleteModule =
{
  tucan_id: string,
  tucan_last_checked: string,
  title: string,
  module_id: string,
  credits: number,
  content: string,
}
export type CourseEvent =
{
  course: number[],
  timestamp_start: string,
  timestamp_end: string,
  room: string,
  teachers: string,
}
export type CourseGroup =
{
  tucan_id: string,
  course: string,
  title: string,
  done: boolean,
}
export type CourseGroupEvent =
{
  course: number[],
  timestamp_start: string,
  timestamp_end: string,
  room: string,
  teachers: string,
}
export type Exam =
{
  tucan_id: string,
  exam_type: string,
  semester: string,
  exam_time_start: string | null,
  exam_time_end: string | null,
  registration_start: string,
  registration_end: string,
  unregistration_start: string,
  unregistration_end: string,
  examinator: string | null,
  room: string | null,
  done: boolean,
}
export type Login =
{
  username: string,
  password: string,
}
export type LoginResult =
{
  success: boolean,
}
export type MaybeCompleteCourse =
 | { type: "Partial", value: PartialCourse }
 | { type: "Complete", value: CompleteCourse }

export type MaybeCompleteModule =
 | { type: "Partial", value: PartialModule }
 | { type: "Complete", value: CompleteModule }

export type ModuleExamType =
{
  module_id: number[],
  exam_type: string,
  required: boolean,
  weight: number,
}
export type ModuleMenu =
{
  tucan_id: string,
  tucan_last_checked: string,
  name: string,
  done: boolean,
  parent: string,
}
export type ModuleMenuPathPart =
{
  parent: number[] | null,
  tucan_id: string,
  name: string,
  leaf: boolean,
}
export type ModuleMenuResponse =
{
  module_menu: ModuleMenu,
  entries: Registration,
  path: ModuleMenuPathPart[][],
}
export type ModuleResponse =
{
  module: CompleteModule,
  courses: MaybeCompleteCourse[],
  exam_types: ModuleExamType[],
  path: ModuleMenuPathPart[][],
}
export type PartialCourse =
{
  tucan_id: string,
  tucan_last_checked: string,
  title: string,
  course_id: string,
}
export type PartialModule =
{
  tucan_id: string,
  tucan_last_checked: string,
  title: string,
  module_id: string,
}
export type Registration =
{
  submenus: ModuleMenu[],
  modules_and_courses: [MaybeCompleteModule, MaybeCompleteCourse[]][],
}
export type SearchResult =
{
  tucan_id: string,
  title: string,
  excerpt: string,
  rank: number,
}
export type VVMenuItem =
{
  tucan_id: string,
  tucan_last_checked: string,
  name: string,
  done: boolean,
  parent: string | null,
}
export type VVMenuPathPart =
{
  parent: string | null,
  tucan_id: string,
  name: string,
  leaf: boolean,
}
export type WithTucanUrlTW9kdWxlTWVudVJlc3BvbnNl =
{
  tucan_url: string,
  inner: ModuleMenuResponse,
}
export type WithTucanUrlTW9kdWxlUmVzcG9uc2U =
{
  tucan_url: string,
  inner: ModuleResponse,
}
export type WithTucanUrlTWF5YmVDb21wbGV0ZU1vZHVsZVtd =
{
  tucan_url: string,
  inner: MaybeCompleteModule[],
}
export type WithTucanUrlW01heWJlQ29tcGxldGVDb3Vyc2VbXSwgQ291cnNlR3JvdXBbXV0 =
{
  tucan_url: string,
  inner: [MaybeCompleteCourse[], CourseGroup[]],
}
export type WithTucanUrlW0NvbXBsZXRlQ291cnNlLCBDb3Vyc2VHcm91cCwgQ291cnNlR3JvdXBFdmVudFtdLCBzdHJpbmdd =
{
  tucan_url: string,
  inner: [CompleteCourse, CourseGroup, CourseGroupEvent[], string],
}
export type WithTucanUrlW0NvbXBsZXRlQ291cnNlLCBDb3Vyc2VHcm91cFtdLCBDb3Vyc2VFdmVudFtdLCBNYXliZUNvbXBsZXRlTW9kdWxlW10sIHN0cmluZ10 =
{
  tucan_url: string,
  inner: [CompleteCourse, CourseGroup[], CourseEvent[], MaybeCompleteModule[], string],
}
export type WithTucanUrlW0V4YW0sIE1heWJlQ29tcGxldGVNb2R1bGVbXSwgTWF5YmVDb21wbGV0ZUNvdXJzZVtdXQ =
{
  tucan_url: string,
  inner: [Exam, MaybeCompleteModule[], MaybeCompleteCourse[]],
}
export type WithTucanUrlW1ZWTWVudUl0ZW0sIFZWTWVudUl0ZW1bXSwgTWF5YmVDb21wbGV0ZUNvdXJzZVtdLCBWVk1lbnVQYXRoUGFydFtdW11d =
{
  tucan_url: string,
  inner: [VVMenuItem, VVMenuItem[], MaybeCompleteCourse[], VVMenuPathPart[][]],
}
export type WithTucanUrlW1tNYXliZUNvbXBsZXRlTW9kdWxlLCBFeGFtXVtdLCBbTWF5YmVDb21wbGV0ZUNvdXJzZSwgRXhhbV1bXV0 =
{
  tucan_url: string,
  inner: [[MaybeCompleteModule, Exam][], [MaybeCompleteCourse, Exam][]],
}