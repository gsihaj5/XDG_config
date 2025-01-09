local utils = require 'neo-tree.utils'
local renderer = require 'neo-tree.ui.renderer'
local events = require 'neo-tree.events'
local manager = require 'neo-tree.sources.manager'
local fs_scan = require 'neo-tree.sources.filesystem.lib.fs_scan'

-- Event to update cursor node when cursor is moved
local cursor_moved = 'cursor_moved'
events.define_autocmd_event(cursor_moved, { 'CursorMoved' })
local cursor_event = function(handler)
    return { event = cursor_moved, id = 'update_cursor_node', handler = handler }
end

---- Helper functions ----

-- Determine if current cursor node is descendent of the seleced node
local cursor_node_is_descendant = function(tree)
    if not tree.cursor_node then return false end
    local curnode = tree.cursor_node
    while true do
        if curnode:get_id() == tree:get_node():get_id() then return true end
        if vim.tbl_contains(tree.nodes.root_ids, curnode:get_id()) then return false end
        curnode = tree.nodes.by_id[curnode:get_parent_id()]
    end
end

-- Set the cursor node position if it is not set
-- and handle removing it when needed
local should_update_cursor = true
local update_cursor_node = function(state)
    if state.tree.cursor_node then return end
    state.tree.cursor_node = state.tree:get_node()
    manager.subscribe(state.name, cursor_event(function()
        if not should_update_cursor then return end
        local should_remove = state.name ~= vim.b.neo_tree_source
        should_remove = should_remove or not cursor_node_is_descendant(state.tree)
        should_remove = should_remove or state.tree:get_node():is_expanded()
        if should_remove then
            state.tree.cursor_node = nil
            manager.unsubscribe(state.name, cursor_event())
        end
    end))
end

-- Recursively open all expandable nodes with given ancestor,
-- including unloaded files and directories in the filesystem source.
-- NOTE: This seems to work, but it's pretty ugly and probably
--       isn't as efficient as it could be.
local num_started, num_done
local function scan_all(state, parent_id, callback, rec)
    if not rec then num_started, num_done = 0, 0 end
    num_started = num_started + 1
    local parent = state.tree.nodes.by_id[parent_id]
    if state.name == 'filesystem' and parent.loaded == false then
        fs_scan.get_items(state, parent_id, nil, function()
            for _, id in ipairs(parent:get_child_ids()) do
                if state.tree.nodes.by_id[id].type == 'directory' then
                    scan_all(state, id, callback, true)
                end
            end
            num_done = num_done + 1
            if num_started == num_done then callback() end
        end)
    else
        if utils.is_expandable(parent) and not parent:is_expanded() then
            parent:expand()
        end
        for _, id in ipairs(parent:get_child_ids()) do
            if utils.is_expandable(state.tree.nodes.by_id[id]) then
                scan_all(state, id, callback, true)
            end
        end
        num_done = num_done + 1
        if num_started == num_done then callback() end
    end
end

---- Commands ----

local commands = {}

-- Used for hjkl navigation
commands.expand_node = function(state)
    local node = state.tree:get_node()
    if not utils.is_expandable(node) or node:is_expanded() then return end
    state.commands.toggle_node(state)

    local focus_child = function()
        if node:has_children() then
            renderer.focus_node(state, node:get_child_ids()[1])
        end
    end
    if state.name == 'filesystem' and node.loaded == false then
        fs_scan.get_items(state, node:get_id(), nil, focus_child)
    else
        focus_child()
    end
end

commands.open_fold = function(state)
    local tree = state.tree
    local node = tree:get_node()
    if not utils.is_expandable(node) or node:is_expanded() then return end
    state.commands.toggle_node(state)

    if tree.cursor_node and tree.cursor_node:get_id() ~= node:get_id() then
        local curnode = tree.cursor_node
        while curnode:get_parent_id() ~= node:get_id() do
            curnode = tree.nodes.by_id[curnode:get_parent_id()]
        end
        renderer.focus_node(state, curnode:get_id())
    end
end

commands.open_folds_rec = function(state)
    local tree = state.tree
    local node = tree:get_node()
    if not utils.is_expandable(node) or node:is_expanded() then return end

    should_update_cursor = false
    scan_all(state, node:get_id(), function()
        if tree.cursor_node then
            renderer.focus_node(state, tree.cursor_node:get_id())
        end
        tree:render()
        should_update_cursor = true
    end)
end

commands.close_fold = function(state)
    update_cursor_node(state)
    state.commands.close_node(state)
end

commands.close_folds_rec = function(state)
    update_cursor_node(state)
    while not vim.tbl_contains(state.tree.nodes.root_ids, state.tree:get_node():get_id()) do
        state.commands.close_node(state)
    end
end

commands.toggle_fold = function(state)
    local node = state.tree:get_node()
    if utils.is_expandable(node) then
        if node:is_expanded() then
            commands.close_fold(state)
        else
            commands.open_fold(state)
        end
    else
        commands.close_fold(state)
    end
end

commands.toggle_folds_rec = function(state)
    local node = state.tree:get_node()
    if utils.is_expandable(node) then
        if node:is_expanded() then
            commands.close_folds_rec(state)
        else
            commands.open_folds_rec(state)
        end
    else
        commands.close_folds_rec(state)
    end
end

commands.fold_view_cursor = function(state)
    local tree = state.tree
    if tree.cursor_node then
        renderer.focus_node(state, tree.cursor_node:get_id())
        tree:render()
    end
end

commands.close_all_folds = function(state)
    update_cursor_node(state)
    state.commands.close_all_nodes(state)
    state.commands.close_node(state)
end

commands.expand_all_folds = function(state)
    local tree = state.tree
    local node = tree:get_node()
    should_update_cursor = false
    scan_all(state, tree.nodes.root_ids[1], function()
        if tree.cursor_node then
            renderer.focus_node(state, tree.cursor_node:get_id())
        else
            renderer.focus_node(state, node:get_id())
        end
        tree:render()
        should_update_cursor = true
    end)
end

commands.focus_fold_start = function(state)
    local parent_id = state.tree:get_node():get_parent_id()
    if parent_id then renderer.focus_node(state, parent_id) end
end

commands.focus_fold_end = function(state)
    local node = state.tree:get_node()
    local last
    if utils.is_expandable(node) and node:is_expanded() then
        if node:has_children() then
            local children = node:get_child_ids()
            last = children[#children]
        end
    elseif node:get_parent_id() then
        local siblings = state.tree.nodes.by_id[node:get_parent_id()]:get_child_ids()
        last = siblings[#siblings]
    end
    if last then renderer.focus_node(state, last) end
end

commands.focus_next_fold_start = function(state, node, dont_check_children)
    node = node or state.tree:get_node()
    local nodes = state.tree.nodes.by_id

    if not dont_check_children and utils.is_expandable(node) and node:has_children() then
        local first_child_id = node:get_child_ids()[1]
        if utils.is_expandable(nodes[first_child_id]) then
            renderer.focus_node(state, first_child_id)
            return
        end
        commands.focus_next_fold_start(state, nodes[first_child_id])
    end

    local parent_id = node:get_parent_id()
    if not parent_id then return end

    local below_node = false
    for _, sibling_id in ipairs(nodes[parent_id]:get_child_ids()) do
        if not below_node then
            if sibling_id == node:get_id() then below_node = true end
        elseif utils.is_expandable(nodes[sibling_id]) then
            renderer.focus_node(state, sibling_id)
            return
        end
    end
    commands.focus_next_fold_start(state, nodes[parent_id], true)
end

commands.focus_prev_fold_end = function(state, node)
    node = node or state.tree:get_node()
    local nodes = state.tree.nodes.by_id

    local parent_id = node:get_parent_id()
    if not parent_id then return end

    local prev_fold_end
    for _, sibling_id in ipairs(nodes[parent_id]:get_child_ids()) do
        if sibling_id == node:get_id() then break end
        if utils.is_expandable(nodes[sibling_id]) then
            prev_fold_end = sibling_id
        end
    end
    if prev_fold_end then
        while nodes[prev_fold_end]:is_expanded() and nodes[prev_fold_end]:has_children() do
            local children = nodes[prev_fold_end]:get_child_ids()
            prev_fold_end = children[#children]
        end
        renderer.focus_node(state, prev_fold_end)
    else
        commands.focus_prev_fold_end(state, nodes[parent_id])
    end
end

return commands
