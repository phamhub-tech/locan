import { ProjectShallowModel } from "~/pages/projects/_models/project"

export default definePayloadPlugin((_) => {
	definePayloadReducer('ProjectShallowModel', (value) => value instanceof ProjectShallowModel && value.toJson())
	definePayloadReviver('ProjectShallowModel', (value) => ProjectShallowModel.fromJson(value))
})
